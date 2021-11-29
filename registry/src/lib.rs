use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen, AccountId, PanicOnDefault, Balance};
use near_sdk::collections::UnorderedMap;

near_sdk::setup_alloc!();
const STORAGE_DEPOSIT: Balance = 2_630_000_000_000_000_000_000; // 1e24yN, 0.5N .00263

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Registry {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    pub registrations: UnorderedMap<u64, AccountId>,
    dsnp_id_index: u64,
}

#[near_bindgen]
impl Registry {

    // pub fn get_num(&self, dsnp_id: u32) -> Option<String> {
    //     return self.registrations.get(&dsnp_id).cloned();
    // }

    #[init]
    pub fn new() -> Self {
        // Initializing `status_updates` with unique key prefix.
        Self {
            registrations: UnorderedMap::new(b"s".to_vec()),
            dsnp_id_index: 0,
        }
    }

    // Register a new dsnpId
    // param account_id - account id to register (contains handle)
    // saves dsnp id to map of handle records
    #[payable]
    pub fn register(&mut self, account_id: AccountId) -> u64 {
        println!("deposit {}, balance limit {}", env::attached_deposit() , STORAGE_DEPOSIT);
        assert!(
            env::attached_deposit() < STORAGE_DEPOSIT,
            "Attached deposit must be greater than INITIAL_BALANCE of .5 NEAR"
        );
        self.dsnp_id_index+=1;
        self.registrations.insert(&self.dsnp_id_index, &account_id );
        env::log(format!("Registered {} to DSNP ID: {}", account_id, self.dsnp_id_index).as_bytes());
        return self.dsnp_id_index;
    }

    pub fn change_address(&mut self, dsnp_id: u64, new_account_id: AccountId) {
        self.registrations.insert(&dsnp_id, &new_account_id) ;
        env::log(format!("Registered {} to DSNP ID: {}", new_account_id, dsnp_id).as_bytes());
    }

    pub fn get_address(self, dsnp_id: u64) -> Option<AccountId> {
        return self.registrations.get(&dsnp_id);
    }

    pub fn resolve_registration(self, account_id: AccountId) -> u64 {
        let keys: Vec<u64> = self.registrations.iter().filter_map(|(key, val)| if val == account_id { Some(key)} else {None}).collect();
        if keys.len() > 1 || keys.len() == 0 {
            return 0;
        } else {
            return keys[0];
        }

    }

    /// Measure the storage an registry will take and need to provide
    pub fn measure_account_storage_usage(&mut self) -> u64 {
        let initial_storage_usage = env::storage_usage();
        // Create a temporary, dummy entry and measure the storage used.
        let tmp_account_id = "a".repeat(64);
        let tmp_dsnp_id = 1;
        self.registrations.insert(&tmp_dsnp_id, &tmp_account_id);
        let usage = env::storage_usage() - initial_storage_usage;
        // Remove the temporary entry.
        self.registrations.remove(&tmp_dsnp_id);
        return usage;
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

    // // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn new_registration() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Registry::new();
        contract.register("ashley.dsnp.testnet".to_string());
        let received_id = contract.resolve_registration("ashley.dsnp.testnet".to_string());
        assert_eq!(1, received_id);
    }

    #[test] 
    fn change_address() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Registry::new();
        let dsnp_id = contract.register("potato.dsnp.testnet".to_string());
        contract.change_address(dsnp_id, "burrito.dsnp.testnet".to_string());
        assert_eq!(dsnp_id, contract.resolve_registration("burrito.dsnp.testnet".to_string()));
    }
}