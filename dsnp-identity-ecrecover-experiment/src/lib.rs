use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sys as other_sys;
use std::convert::TryInto;
extern crate hex;

near_sdk::setup_alloc!();

// pub extern "C" fn cool_function(
//   i: cty::c_int,
//   c: cty::c_char,
//   cs: *mut CoolStruct
// );

// pub fn ecrecover(
//   hash_len: u64,
//   hash_ptr: u64,
//   sig_len: u64,
//   sig_ptr: u64,
//   v: u64,
//   malleability_flag: u64,
//   register_id: u64,
// ) -> u64;

// #[derive(Deserialize)]
// struct ECData {
//     #[serde(with = "hex::serde")]
//     message: [u8; 32],
//     #[serde(with = "hex::serde")]
//     signature: [u8; 64],
// }

pub fn ecrecover(
  hash_buffer: [u8; 32],
  sig_buffer: [u8; 64],
  // v: u64,
) -> u64 {
  unsafe { other_sys::ecrecover(32, hash_buffer.as_ptr() as _, 64, sig_buffer.as_ptr() as _, 0, 0, 0) } 
}


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Store {
}

#[near_bindgen]
impl Store {
  pub fn verify (message_hash: String, signature: String) -> String {
    let hash_decode: [u8; 32] = match hex::decode(&message_hash).unwrap().try_into() {
      Ok(hc) => hc,
      Err(o) => panic!("Expected a Vec of length {} but it was {}", 32, o.len()),
    };

    let hash_sig: [u8; 64]  = match hex::decode(&signature).unwrap().try_into() {
      Ok(hs) => hs,
      Err(o) => panic!("Expected a Vec of length {} but it was {}", 64, o.len()),
    };

    env::log(format!("public key {}", hex::encode(env::signer_account_pk())).as_bytes());

    // let data = ECData { signature: hashSig , message: hashDecode };
    ecrecover(hash_decode, hash_sig );

    match env::read_register(0) {
      None => panic!("Could not read from register 0"),
      Some(ba) => hex::encode(ba),
    }
  }
}