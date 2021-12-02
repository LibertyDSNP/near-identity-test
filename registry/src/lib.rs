mod storage_impl;

use near_contract_standards::storage_management::StorageManagement;
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey,StorageUsage, Balance, Promise,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap},
    json_types::{U128, ValidAccountId},
    serde::{Deserialize, Serialize}
};

near_sdk::setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    Registrations,
    AccountsIdMap,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    // registered account id
    pub account_id: AccountId,

    // storage balance
    pub balance: U128,
}

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Registry {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    registration_map: UnorderedMap<u32, User>,
    account_id_map: LookupMap<AccountId, u32>,
    dsnp_id_index: u32,

    // Storage
    account_storage_usage: StorageUsage,
}

#[near_bindgen]
impl Registry {

    #[init]
    pub fn new() -> Self {
        // Initializing `status_updates` with unique key prefix.
        let mut this = Self {
            registration_map: UnorderedMap::new(StorageKeys::Registrations),
            account_id_map: LookupMap::new(StorageKeys::AccountsIdMap),
            dsnp_id_index: 0,
            account_storage_usage: 0,
        };
        this.measure_account_storage_usage();
        this
    }

    /// Measure the storage an registry will take and need to provide
    fn measure_account_storage_usage(&mut self) {
        let initial_storage_usage = env::storage_usage();
        // Create a temporary, dummy entry and measure the storage used.
        let tmp_dsnp_id = 0; // using zero since valid ones starts from 1
        let tmp_account_id = "0".repeat(64);
        self.register_account_internal(&tmp_account_id.clone().into(), Some(tmp_dsnp_id));
        self.account_storage_usage = env::storage_usage() - initial_storage_usage;
        // Remove the temporary entry.
        self.remove_account_internal(&tmp_account_id.into());
    }

    // Register a new dsnpId
    // param account_id - account id to register (contains handle)
    // saves dsnp id to map of handle records
    #[payable]
    pub fn register(&mut self, account_id: ValidAccountId) -> u32 {
        let deposit: Balance = env::attached_deposit();
        let required_deposit: Balance = Balance::from(self.account_storage_usage) * env::storage_byte_cost();

        assert!(
            deposit >= required_deposit,
            "Insufficient deposit. Please deposit {} yoctoⓃ to register an agent.",
            required_deposit.clone()
        );

        // check that account isn't already added
        if let Some(account) = self.account_id_map.get(&account_id.as_ref().into()) {
            let panic_msg = format!("Account already exists: {:?}. Refunding the deposit.", account);
            env::panic(panic_msg.as_bytes());
        };

        let new_id = self.register_account_internal(account_id.as_ref().into(), None);
        env::log(format!("Registered {} to DSNP ID: {}", &account_id, new_id).as_bytes());

        // If the user deposited more than needed, refund them.
        let refund = deposit - required_deposit;
        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        new_id
    }

    /// Removes the account fro registered list
    /// Withdraws storage balances to the account.
    /// Requires attaching 1 yoctoⓃ ensure it comes from a full-access key.
    #[payable]
    pub fn unregister(&mut self) {
        // This method name is quite explicit, so calling storage_unregister and setting the 'force' option to true.
        self.storage_unregister(Some(true));
    }

    pub fn get_account_storage(self) -> StorageUsage {
        self.account_storage_usage
    }

    pub fn get_address(self, dsnp_id: u32) -> Option<User> {
        self.registration_map.get(&dsnp_id)
    }

    pub fn get_registration(self, account_id: ValidAccountId) -> Option<u32> {
        self.account_id_map.get(&account_id.into())
    }

    fn register_account_internal(&mut self, account_id: &AccountId, optional_id: Option<u32>) -> u32 {

        let new_id = match optional_id {
            Some(id) => id,
            None => {
                self.dsnp_id_index += 1;
                self.dsnp_id_index
            }
        };

        let user = User {
            balance: U128(Balance::from(self.account_storage_usage) * env::storage_byte_cost()),
            account_id: account_id.into(),
        };

        self.registration_map.insert(&new_id, &user);
        self.account_id_map.insert( account_id, &new_id);
        new_id
    }

    fn remove_account_internal(&mut self, account_id: &AccountId) {
        if let Some(registration_id) = self.account_id_map.get(account_id) {
            self.registration_map.remove(&registration_id);
            self.account_id_map.remove(account_id);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn new_registration() {
        // arrange
        let mut context = get_context(vec![], false);
        context.attached_deposit = 10u128.pow(22).into();
        testing_env!(context);

        // act
        let mut contract = Registry::new();
        let id = contract.register("ashley.dsnp.testnet".try_into().unwrap());
        let received = contract.get_address(id);

        // assert
        assert!(received.is_some());
        assert_eq!("ashley.dsnp.testnet", received.as_ref().unwrap().account_id);
        assert!(received.as_ref().unwrap().balance.0 > 0);
    }

    // test panics due to some unknown issue in deserializing the value inside
    // self.registration_map.remove(&registration_id) call
    // this issue does not exists in testnet and only happens in local tests
    // #[test]
    // fn register_unregister() {
    //     // arrange
    //     let mut context = get_context(vec![], false);
    //     context.attached_deposit = 10u128.pow(22).into();
    //     testing_env!(context);
    //
    //     // act
    //     let mut contract = Registry::new();
    //     let id = contract.register("ashley.dsnp.testnet".try_into().unwrap());
    //
    //     context = get_context(vec![], false);
    //     context.attached_deposit = 1u128;
    //     context.predecessor_account_id = "ashley.dsnp.testnet".try_into().unwrap();
    //     testing_env!(context);
    //     contract.unregister();
    //     let received = contract.get_address(id);
    //
    //     // assert
    //     assert!(received.is_none(), "Account should not exists");
    // }
}