// use std::str::FromStr;
// use bip39::{Language, Mnemonic, Error};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::*;
use near_sdk::{
    // bs58,
    PanicOnDefault};
use near_sdk::{env, near_bindgen, Balance, Promise};
use near_sdk::json_types::Base58PublicKey;

near_sdk::setup_alloc!();

const INITIAL_BALANCE: Balance = 500_000_000_000_000_000_000_000; // 1e24yN, 0.5N

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ImplicitAccount {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    // latest_dsnp_id: u32,
    pub accounts: UnorderedMap<String, String>,
}

#[near_bindgen]
impl ImplicitAccount {

    #[init]
    pub fn new() -> Self {
        Self {
            // latest_dsnp_id: 0,
            accounts: UnorderedMap::new(b"s".to_vec()),
        }
    }

    // pub fn get_latest_dsnp(&self) -> u32 {
    //     return self.latest_dsnp_id;
    // }

    // pub fn new_registration(&mut self) {
    //     // note: adding one like this is an easy way to accidentally overflow
    //     // real smart contracts will want to have safety checks
    //     // e.g. self.val = i8::wrapping_add(self.val, 1);
    //     // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
    //     self.latest_dsnp_id += 1;
    //     let log_message = format!("Created DSNP Id: {}", self.latest_dsnp_id);
    //     //self.registrations.insert(self.latest_dsnp_id, handle);
    //     env::log(log_message.as_bytes());
    //     //after_registration();
    // }

    pub fn get_accounts_count(&self) -> u64 {
        return self.accounts.len();
    }

    // #[payable]
    // pub fn create_and_transfer_account(&mut self) {
    //     assert!(
    //         env::attached_deposit() >= INITIAL_BALANCE,
    //         "Attached deposit must be greater than INITIAL_BALANCE of .5 NEAR"
    //     );
    //     let new_public_key = ImplicitAccount::generate_pub_key();
    //     let public_key_string = bs58::encode(&new_public_key).into_string();
    //     let implicit_id = &hex::encode(new_public_key)[2..];
    //     env::log(format!("implicit id: {:?}", &implicit_id).as_bytes());
    //     let key = public_key_string.clone();
    //     Promise::new(implicit_id.into())
    //         .transfer(INITIAL_BALANCE);
    //     self.accounts.insert(&String::from(implicit_id), &key);
    //     println!("created new account {:?} with key {:?}", &implicit_id, key);
    // }


    //before calling this you must generate a keypair from a 12 word phrase 
    // generate an account from a public key
    #[payable]
    pub fn generate_implicit_account_from_key(&mut self, new_public_key: Base58PublicKey) {
        env::log(format!("deposit {}, balance limit {}", env::attached_deposit() , INITIAL_BALANCE).as_bytes());
        assert!(
            env::attached_deposit() >= INITIAL_BALANCE,
            "Attached deposit must be greater than INITIAL_BALANCE of .5 NEAR"
        );
        env::log(format!("key received: {:?}", new_public_key).as_bytes());
        let key = new_public_key.clone();
        let implicit_id = &hex::encode(new_public_key.0)[2..];
        env::log(format!("implicit id: {:?}", &implicit_id).as_bytes());
        Promise::new(implicit_id.into()) // skipping starting 00
            .transfer(INITIAL_BALANCE); // sending tokens will create the implicit account with used public key as a full access key
        self.accounts.insert(&String::from(implicit_id), &format!("{:?}", &key));
        env::log(format!("created new account {:?} with key {:?}", &implicit_id, key).as_bytes());
    }

    //from https://github.com/near/near-cli-rs/blob/12dfa268b72ba4778aedf898361f46c5136fc75f/src/commands/add_command/implicit_account/generate_keypair/mod.rs
    // fn generate_pub_key() -> ed25519_dalek::PublicKey {
    //     let seed_phrase_hd_path = slip10::BIP32Path::from_str("m/44'/397'/0'").unwrap();
    //
    //     let (master_seed_phrase, master_seed) = ImplicitAccount::generate_seed_phrase();
    //     env::log(format!("generated seed phrase {:?}", master_seed_phrase).as_bytes());
    //
    //     let derived_private_key = slip10::derive_key_from_path(
    //         &master_seed,
    //         slip10::Curve::Ed25519,
    //         &seed_phrase_hd_path,
    //     ).map_err(|err| {
    //         // color_eyre::Report::msg(format!("Key derivation from path failed: {:?}", err))
    //         env::log(format!("Error: {:?}", &err).as_bytes());
    //     })
    //     .unwrap();
    //
    //     let secret_keypair = {
    //         let secret = ed25519_dalek::SecretKey::from_bytes(&derived_private_key.key).unwrap();
    //         let public = ed25519_dalek::PublicKey::from(&secret);
    //         ed25519_dalek::Keypair { secret, public }
    //     };
    //     return secret_keypair.public;
    // }

    // fn generate_seed_phrase() -> (String, [u8; 64]) {
    //     let mnemonic = ImplicitAccount::generate_in_with(Language::English,12).unwrap();
    //     // let mnemonic = bip39::Mnemonic::generate_in_with(&mut rand::rngs::ThreadRng(),Language::English,12).unwrap();
    //     let mut master_seed_phrase = String::new();
    //     for (index, word) in mnemonic.word_iter().enumerate() {
    //         if index != 0 {
    //             master_seed_phrase.push(' ');
    //         }
    //         master_seed_phrase.push_str(word);
    //     }
    //     (master_seed_phrase, mnemonic.to_seed(""))
    // }
    //
    // fn generate_in_with(
    //     language: Language,
    //     word_count: usize,
    // ) -> Result<Mnemonic, Error>
    // {
    //     let entropy_bytes = (word_count / 3) * 4;
    //     let mut entropy = [0u8; (24 / 3) * 4];
    //
    //     getrandom::getrandom(&mut entropy[0..entropy_bytes]);
    //     // my_rng.fill_bytes( &mut entropy[0..entropy_bytes]);
    //     Mnemonic::from_entropy_in(language, &entropy[0..entropy_bytes])
    // }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
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
            attached_deposit: 700_000_000_000_000_000_000_000,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn create_and_transfer_account() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = ImplicitAccount::new();

        //near generate-key --seedPhrase "silk thank piano other pull forum sure group dignity spend sun potato"
        //Key pair with ed25519:2uy3CUJo3tB3fvDoozoenrYR2Ub4zQBVMx98VkjYpXue public key for an account "1c6d40519f3ccf17e81bab51cb1f7daeeda6cc2c93ade2d9510f2dfd9bad2a6d"
        contract.generate_implicit_account_from_key( "ed25519:2uy3CUJo3tB3fvDoozoenrYR2Ub4zQBVMx98VkjYpXue".try_into().unwrap());
        // confirm that we added an account to the array
        assert_eq!(1, contract.get_accounts_count());
    }
}