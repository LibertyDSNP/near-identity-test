use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Gas, PromiseOrValue};
use near_sdk::json_types::{Base58CryptoHash};

pub const VERSION: u32 = 1;

/// Raw type for 32 bytes of the hash.
pub type CryptoHash = [u8; 32];

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(30_000_000_000_000);

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Identity {
    handle: String,
}

impl Default for Identity {
    fn default() -> Self {
        env::panic_str("SubAccountFactory should be initialized before usage")
    }
}

#[near_bindgen]
impl Identity {
    #[init]
    pub fn new(handle: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            handle,
        }
    }

    /// Should only be called by this contract on migration.
    /// This is NOOP implementation. KEEP IT if you haven't changed contract state.
    /// If you have changed state, you need to implement migration from old state (keep the old struct with different name to deserialize it first).
    /// After migrate goes live on MainNet, return this implementation for next updates.
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "ERR_NOT_ALLOWED"
        );
        let this: Identity = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        this
    }

    pub fn upgrade(hash: &Base58CryptoHash) -> PromiseOrValue<()> {
        upgrade_self(&CryptoHash::from(hash.clone()));
        PromiseOrValue::Value(())
    }

    pub fn get_handle(&self) -> &String {
        &self.handle
    }

    pub fn get_version() -> u32 {
        VERSION
    }
}

// Stores attached data into blob store and returns hash of it.
/// Implemented to avoid loading the data into WASM for optimal gas usage.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn store_blob() {
    use near_sdk::sys;
    env::setup_panic_hook();
    // let mut identity: Identity = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
    let identity: Identity = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
    unsafe {
        // Load input into register 0.
        sys::input(0);
        // Compute sha256 hash of register 0 and store in 1.
        sys::sha256(u64::MAX as _, 0 as _, 1);
        // Check if such blob already stored.
        assert_eq!(
            sys::storage_has_key(u64::MAX as _, 1 as _),
            0,
            "ERR_ALREADY_EXISTS"
        );
        // Get length of the input argument and check that enough $NEAR has been attached.
        let blob_len = near_sdk::sys::register_len(0);
        let storage_cost = ((blob_len + 32) as u128) * env::storage_byte_cost();
        env::log_str(format!("storage cost {}", storage_cost).as_str());
        // assert!(
        //     env::attached_deposit() >= storage_cost,
        //     "ERR_NOT_ENOUGH_DEPOSIT:{}",
        //     storage_cost
        // );
        // identity.locked_amount += storage_cost;
        // Store value of register 0 into key = register 1.
        sys::storage_write(u64::MAX as _, 1 as _, u64::MAX as _, 0 as _, 2);
        // Load register 1 into blob_hash and save into LookupMap.
        let blob_hash = [0u8; 32];
        sys::read_register(1, blob_hash.as_ptr() as _);
        // identity
        //     .blobs
        //     .insert(&blob_hash, &env::predecessor_account_id());
        // Return from function value of register 1.
        let blob_hash_str = near_sdk::serde_json::to_string(&Base58CryptoHash::from(blob_hash))
            .unwrap()
            .into_bytes();
        sys::value_return(blob_hash_str.len() as _, blob_hash_str.as_ptr() as _);
    }
    env::state_write(&identity);
}

/// Self upgrade, optimizes gas by not loading into memory the code.
pub fn upgrade_self(hash: &[u8]) {
    let current_id = env::current_account_id();
    let method_name = "migrate".as_bytes().to_vec();
    let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE_SELF_DEPLOY;
    use near_sdk::sys;
    unsafe {
        // Load input (wasm code) into register 0.
        sys::storage_read(hash.len() as _, hash.as_ptr() as _, 0);
        // schedule a Promise tx to this same contract
        let promise_id = sys::promise_batch_create(
            current_id.as_bytes().len() as _,
            current_id.as_bytes().as_ptr() as _,
        );
        // 1st item in the Tx: "deploy contract" (code is taken from register 0)
        sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
        // 2nd item in the Tx: call this_contract.migrate() with remaining gas
        sys::promise_batch_action_function_call(
            promise_id,
            method_name.len() as _,
            method_name.as_ptr() as _,
            0 as _,
            0 as _,
            0 as _,
            attached_gas.0,
        );
    }
}