use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Identity {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    latest_dsnp_id: u32,
}

#[near_bindgen]
impl Identity {

    // pub fn get_num(&self, dsnp_id: u32) -> Option<String> {
    //     return self.registrations.get(&dsnp_id).cloned();
    // }

    pub fn get_latest_dsnp(&self) -> u32 {
        return self.latest_dsnp_id;
    }

    pub fn new_registration(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i8::wrapping_add(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
        self.latest_dsnp_id += 1;
        let log_message = format!("Created DSNP Id: {}", self.latest_dsnp_id);
        //self.registrations.insert(self.latest_dsnp_id, handle);
        env::log(log_message.as_bytes());
        after_registration();
    }
}

fn after_registration() {
    env::log("Created a new registration! Much excite!".as_bytes());
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
    // #[test]
    // fn new_registration() {
    //     // set up the mock context into the testing environment
    //     let context = get_context(vec![], false);
    //     testing_env!(context);
    //     // instantiate a contract variable with the counter at zero
    //     let mut contract = Identity { latest_dsnp_id: 0, registrations: HashMap::new() };
    //     contract.new_registration(String::from("potato"));
    //     println!("Value after increment: {}", contract.get_latest_dsnp());
    //     // confirm that we received 1 when calling get_num
    //     assert_eq!(1, contract.get_latest_dsnp());
    // }
}