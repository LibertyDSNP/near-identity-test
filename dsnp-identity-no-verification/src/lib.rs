use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U64;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey,
    Gas, PanicOnDefault,
};

near_sdk::setup_alloc!();

const VERIFICATION_ACCOUNT_ID: &str = "ed25519-verification.testnet";
const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = 5_000_000_000_000;

#[ext_contract(ext_verification)]
trait ExVerification {
    fn ed25519_verification(
        &self,
        signature: String,
        public_key: String,
        serialize_message: String,
    ) -> Option<u64>;
    // fn callback_arg_macro(#[callback] account_id: Option<AccountId>) -> Option<AccountId>;
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_arg_macro(#[callback] is_valid: Option<AccountId>);
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    AccountsIdMap,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AddDelegateParam {
    account_id: AccountId,
    public_key: String,
    end_block: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Delegate {
    end_block: u64,
    nonce: u32,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DelegateStore {
    owner_id: AccountId,
    pub delegates: LookupMap<AccountId, Delegate>,
}

#[near_bindgen]
impl DelegateStore {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let mut this = Self {
            owner_id: owner_id.clone(),
            delegates: LookupMap::new(StorageKeys::AccountsIdMap),
        };

        this.delegates.insert(
            &owner_id,
            &Delegate {
                end_block: 0,
                nonce: 1,
            },
        );

        this
    }

    pub fn get_delegate(&self, account_id: &AccountId) -> (U64, u32) {
        let delegate = self.delegates.get(&account_id).unwrap();
        (delegate.end_block.into(), delegate.nonce)
    }

    pub fn upsert_delegate_via_sig(
        &mut self,
        signature: String,
        sig_public_key: String,
        message: (AccountId, U64, String),
    ) {
        let account_id: AccountId = message.0.clone();
        let end_block: u64 = message.1.into();
        let public_key: String = message.2;
    
        let serialize_message = format!(
            "[\"{}\",\"{}\",\"{}\"]",
            account_id,
            end_block,
            public_key
        );

        let verification_account_id: AccountId = VERIFICATION_ACCOUNT_ID.to_string();
        ext_verification::ed25519_verification(
            signature,
            sig_public_key.clone(),
            serialize_message,
            &verification_account_id,
            NO_DEPOSIT,
            BASE_GAS,
        )
        .then(ext_self::callback_arg_macro(
            &env::current_account_id(),
            0,                 // yocto NEAR to attach to the callback
            5_000_000_000_000, // gas to attach to the callback
        ));

        let delegate = &self.get_delegate(&message.0);
        let nonce: u32 = delegate.1.into();

        self.delegates.insert(
            &message.0,
            &Delegate {
                end_block: message.1.into(),
                nonce: nonce + 1,
            },
        );
    }

    #[private]
    pub fn callback_arg_macro(&mut self, #[callback] is_valid: Option<u64>) -> bool {
        match is_valid {
            Some(_u) => env::log("YASSSSSS!!!!!!!!!".as_bytes()),
            None => env::log("ooooh shittt".as_bytes()),
        }

        true
    }
}
