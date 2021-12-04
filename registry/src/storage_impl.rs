use crate::{Registry};
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{assert_one_yocto, env, log, AccountId, Balance, Promise};

impl Registry {
    fn internal_storage_balance_of(&self, account_id: &AccountId) -> Option<StorageBalance> {
        if self.account_id_map.contains_key(account_id) {
            // The "available" balance is always zero because the storage isn't
            // variable for this contract.
            let user = self.registration_map.get(&self.account_id_map.get(account_id).unwrap()).unwrap();
            Some(StorageBalance {
                total: user.balance,
                available: 0.into(),
            })
        } else {
            None
        }
    }
}

impl StorageManagement for Registry {
    // `registration_only` doesn't affect the implementation here, as there's no need to add additional
    // storage, so there's only one balance to attach.
    #[allow(unused_variables)]
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        let account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(|| env::predecessor_account_id());
        self.register(account_id.clone().try_into().unwrap());
        self.internal_storage_balance_of(&account_id).unwrap()
    }

    /// While storage_withdraw normally allows the caller to retrieve `available` balance, this
    /// contract sets storage_balance_bounds.min = storage_balance_bounds.max,
    /// which means available balance will always be 0. So this implementation:
    /// * panics if `amount > 0`
    /// * never transfers Ⓝ to caller
    /// * returns a `storage_balance` struct if `amount` is 0
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        assert_one_yocto();
        let predecessor = env::predecessor_account_id();
        if let Some(storage_balance) = self.internal_storage_balance_of(&predecessor) {
            match amount {
                Some(amount) if amount.0 > 0 => {
                    let panic_msg = format!("The amount is greater than the available storage balance. Remember there's a minimum balance needed for an account's storage. That minimum is {}. To unregister an account, use the 'unregister' or 'storage_unregister' with the 'force' option.", self.account_storage_usage);
                    env::panic(panic_msg.as_bytes());
                }
                _ => storage_balance,
            }
        } else {
            env::panic(format!("The account {} is not registered", &predecessor).as_bytes());
        }
    }

    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let force = force.unwrap_or(false);
        if let Some(registration_id) = self.account_id_map.get(&account_id) {
            let user = self.registration_map.get(&registration_id).unwrap();
            let balance = user.balance.0;
            if balance == 0 || force {

                self.remove_account_internal(&account_id.clone());

                // We add 1 to reimburse for the 1 yoctoⓃ used to call this method
                Promise::new(account_id).transfer(balance + 1);
                log!(
                    "Agent has been removed and refunded the storage cost of {}",
                    balance + 1
                );
                true
            } else {
                env::panic(b"Can't unregister the account with the positive balance. Must use the 'force' parameter if desired.")
            }
        } else {
            log!("The account {} is not registered", &account_id);
            false
        }
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        let required_storage_balance =
            Balance::from(self.account_storage_usage) * env::storage_byte_cost();
        StorageBalanceBounds {
            min: required_storage_balance.into(),
            max: Some(required_storage_balance.into()),
        }
    }

    fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance> {
        self.internal_storage_balance_of(account_id.as_ref())
    }
}
