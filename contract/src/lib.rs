use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, serde_json::json,
    AccountId
} ; // , setup_alloc};
use near_sdk::collections::LookupMap;
// use near_sdk::json_types::{Base64VecU8, U128};

// use std::collections::HashMap;
// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
// use near_sdk::json_types::{Base64VecU8, U128};
// use near_sdk::serde::{Deserialize, Serialize};
// use near_sdk::{
//     env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
//     PromiseResult, Gas, require, serde_json::json
// };


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<String, String>,
}

impl Default for Welcome {
  fn default() -> Self {
    Self {
      records: LookupMap::new(b"a".to_vec()),
    }
  }
}

#[near_bindgen]
impl Welcome {
    pub fn set_greeting(&mut self, message: String) {
        let signer_account_id = env::signer_account_id();   // initial
        let current_account_id = env::current_account_id(); // contract id
        let predecessor_account_id = env::predecessor_account_id(); // contract id
        

        // Use env::log to record logs permanently to the blockchain!
        // env::log_str(format!("Saving greeting '{}' for account '{}'", message, account_id,).as_bytes());

        env::log_str(
            &json!({
                "method": "set_greeting",
                "params": {
                    "message": message
                },
                "accounts": {
                    "signer_account_id": signer_account_id.to_string(),
                    "current_account_id": current_account_id.to_string(),
                    "predecessor_account_id": predecessor_account_id.to_string(),                    
                }
            })
            .to_string(),
        );

        self.records.insert(&signer_account_id.to_string(), &message);
    }

    // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
    // self.records.get(&account_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn get_greeting(&self, account_id: AccountId) -> String {
        match self.records.get(&account_id.to_string()) {
            Some(greeting) => greeting,
            None => "Hello".to_string(),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder}; 
    use near_sdk::{testing_env, VMContext};
    // use std::convert::TryInto;    

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    // fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    //     VMContext {
    //         current_account_id: "alice_near".try_into().unwrap(),
    //         signer_account_id: "bob_near".try_into().unwrap(),
    //         signer_account_pk: vec![0, 1, 2],
    //         predecessor_account_id: "carol_near".try_into().unwrap(),
    //         input,
    //         block_index: 0,
    //         block_timestamp: 0,
    //         account_balance: 0,
    //         account_locked_balance: 0,
    //         storage_usage: 0,
    //         attached_deposit: 0,
    //         prepaid_gas: 10u64.pow(18),
    //         random_seed: vec![0, 1, 2],
    //         is_view,
    //         output_data_receivers: vec![],
    //         epoch_height: 19,
    //     }
    // }
    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(accounts(0))
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_then_get_greeting() {
        // let context = get_context(vec![], false);
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting(accounts(0))
        );
    }

    #[test]
    fn get_default_greeting() {
        // let context = get_context(vec![], true);
        let context = get_context(false);
        testing_env!(context);
        let contract = Welcome::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting(accounts(1))
        );
    }
}
