use bs58;
use ed25519_dalek::Signature;
use ed25519_dalek::{PublicKey, Verifier};
use hex::FromHex;
use near_sdk::{env::sha256, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
pub struct Ed25519 {}

#[near_bindgen]
impl Ed25519 {
  pub fn ed25519_verify(signature: String, public_key: String, serialize_message: String) -> u64 {
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

// // use the attribute below for unit tests
// #[cfg(test)]
// mod tests {
//   use super::*;
//   use near_sdk::test_utils::{get_logs, VMContextBuilder};
//   use near_sdk::{testing_env, AccountId};

  // part of writing unit tests is setting up a mock context
  // provide a `predecessor` here, it'll modify the default context
//   fn get_context(predecessor: AccountId) -> VMContextBuilder {
//     let mut builder = VMContextBuilder::new();
//     builder.predecessor_account_id(predecessor);
//     builder
//   }

  // TESTS HERE
// }
