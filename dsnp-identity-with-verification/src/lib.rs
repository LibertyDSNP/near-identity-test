use bs58;
use ed25519_dalek::Signature;
use ed25519_dalek::{PublicKey, Verifier};
use hex::FromHex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U64};
use near_sdk::{
  collections::LookupMap, env::sha256, near_bindgen, AccountId,
  BorshStorageKey, PanicOnDefault,
};

use near_sdk::serde::{Deserialize, Serialize};

near_sdk::setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
  AccountsIdMap,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddDelegateParam {
  account_id: AccountId,
  public_key: String,
  end_block: U64,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
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

  pub fn get_delegate(&self, account_id: &AccountId) -> Delegate {
    self.delegates.get(&account_id).unwrap()
  }

  pub fn upsert_delegate_via_sig(
    &mut self,
    signature: String,
    public_key: String,
    message: AddDelegateParam,
  ) {

    let serialize_message = format!(
      "[\"{}\",\"{}\",\"{}\"]",
      message.account_id,
      message.end_block.0.to_string(),
      message.public_key
    );

    /// verify public key exist in delegates
  
    DelegateStore::ed25519_verify(signature, public_key, serialize_message);

    let delegate = &self.get_delegate(&message.account_id);
  
    self.delegates.insert(
      &message.account_id,
      &Delegate {
        end_block: message.end_block.into(),
        nonce: delegate.nonce + 1,
      },
    );
  }

  fn ed25519_verify(signature: String, public_key: String, serialize_message: String) -> u64 {
    let sig_bytes: Vec<u8> = FromHex::from_hex(&signature).unwrap();
    let sig: Signature = Signature::from_bytes(&sig_bytes[..64]).unwrap();

    let public_key = PublicKey::from_bytes(&bs58::decode(public_key).into_vec().unwrap()).unwrap();

    near_sdk::env::log(
      format!(
        "Verifiying validity of signature ('{:?}') for string '{}'...",
        signature, "taco"
      )
      .as_bytes(),
    );

    near_sdk::env::log(format!("json param account:'{}'", serialize_message).as_bytes());

    let hash_message: Vec<u8> = sha256(serialize_message.as_bytes());
    if let Ok(_) = public_key.verify(hash_message.as_slice(), &sig) {
      return 1 as u64;
    }

    panic!("message verification failed");
  }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::test_utils::{get_logs, VMContextBuilder};
  use near_sdk::{testing_env, AccountId};

  // part of writing unit tests is setting up a mock context
  // provide a `predecessor` here, it'll modify the default context
  fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
  }

  // TESTS HERE
}
