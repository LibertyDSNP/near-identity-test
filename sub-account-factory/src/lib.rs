use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::{
    // Base64VecU8,
    U128};
use near_sdk::{assert_self, env, ext_contract, near_bindgen, AccountId, Gas, Promise, PublicKey, Balance};

const CODE: &[u8] = include_bytes!("../identity/rust_counter_tutorial.wasm");

/// Gas spent on the call & account creation.
// const CREATE_CALL_GAS: Gas = Gas(75_000_000_000_000);

/// Gas allocated on the callback.
const ON_CREATE_CALL_GAS: Gas = Gas(10_000_000_000_000);

const FUNCTION_KEY_ALLOWANCE: Balance = 250_000_000_000_000_000_000_000;

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn on_create(
        &mut self,
        account_id: AccountId,
        attached_deposit: U128,
        predecessor_account_id: AccountId,
    ) -> bool;
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SubAccountFactory {
    accounts: UnorderedSet<AccountId>,
    dsnp_id: u64,
}

impl Default for SubAccountFactory {
    fn default() -> Self {
        env::panic_str("SubAccountFactory should be initialized before usage")
    }
}

#[near_bindgen]
impl SubAccountFactory {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            accounts: UnorderedSet::new(b"d".to_vec()),
            dsnp_id: 1000,
        }
    }

    pub fn get_accounts_list(&self) -> Vec<AccountId> {
        self.accounts.to_vec()
    }

    /// Get number of created accounts.
    pub fn get_number_accounts(&self) -> u64 {
        self.accounts.len()
    }

    /// Get accounts in paginated view.
    pub fn get_accounts(&self, from_index: u64, limit: u64) -> Vec<AccountId> {
        let elements = self.accounts.as_vector();
        (from_index..std::cmp::min(from_index + limit, elements.len()))
            .filter_map(|index| elements.get(index))
            .collect()
    }

    #[payable]
    pub fn create(
        &mut self,
        public_key: Option<PublicKey>,
        function_key: Option<PublicKey>,
        deploy: bool,
        // args: Base64VecU8,
    ) -> Promise {
        // todo: should use atomic counter
        self.dsnp_id += 1;
        let account_id: AccountId = format!("{}.{}", self.dsnp_id, env::current_account_id())
            .parse()
            .unwrap();
        let mut promise = Promise::new(account_id.clone())
            .create_account()
            .transfer(env::attached_deposit());
        if deploy {
            promise = promise.deploy_contract(CODE.to_vec())
        }
        if let Some(key) = public_key {
            promise = promise.add_full_access_key(key.into())
        }
        if let Some(key) = function_key {
            promise = promise.add_access_key(key.into(), FUNCTION_KEY_ALLOWANCE, env::current_account_id(),"".into());
        }
        promise
            // TODO: enable when identity has init function
            // .function_call(
            //     "new".to_string(),
            //     args.into(),
            //     0,
            //     env::prepaid_gas() - CREATE_CALL_GAS - ON_CREATE_CALL_GAS,
            // )
            .then(ext_self::on_create(
                account_id,
                U128(env::attached_deposit()),
                env::predecessor_account_id(),
                env::current_account_id(),
                0,
                ON_CREATE_CALL_GAS,
            ))
    }

    pub fn on_create(
        &mut self,
        account_id: AccountId,
        attached_deposit: U128,
        predecessor_account_id: AccountId,
    ) -> bool {
        assert_self();
        if near_sdk::is_promise_success() {
            self.accounts.insert(&account_id);
            true
        } else {
            Promise::new(predecessor_account_id).transfer(attached_deposit.0);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, testing_env_with_promise_results, VMContextBuilder};
    use near_sdk::{testing_env, PromiseResult};

    use super::*;

    #[test]
    fn test_basics() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.current_account_id(accounts(0)).build());
        let mut factory = SubAccountFactory::new();
        testing_env!(context.attached_deposit(10).build());
        factory.create(
            Some(
                "ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp"
                    .parse()
                    .unwrap(),
            ),
            // "{}".as_bytes().to_vec().into(),
        );
        testing_env_with_promise_results(
            context.predecessor_account_id(accounts(0)).build(),
            PromiseResult::Successful(vec![]),
        );
        factory.on_create(
            format!("1000.{}", accounts(0)).parse().unwrap(),
            U128(10),
            accounts(0),
        );
        assert_eq!(
            factory.get_accounts_list(),
            vec![format!("1000.{}", accounts(0)).parse().unwrap()]
        );
        assert_eq!(
            factory.get_accounts(0, 100),
            vec![format!("1000.{}", accounts(0)).parse().unwrap()]
        );
    }
}
