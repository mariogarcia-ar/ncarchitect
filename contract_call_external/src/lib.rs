use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, serde_json::json, Gas, promise_result_as_success,
    AccountId, Promise , PromiseResult //, PromiseOrValue, PanicOnDefault
} ;  
use near_sdk::collections::LookupMap;

// use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, 
//     ext_contract, Gas, PromiseResult, PromiseOrValue};
// use near_sdk::{ext_contract}; // used to handle remote contract invocation
// use near_sdk::{ Promise}; 

use crate::external::*;
mod external;

pub const CALLBACK_GAS: Gas = Gas(5_000_000_000_000);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<String, String>,
    ipfs: LookupMap<String, String>,
}

impl Default for Welcome {
  fn default() -> Self {
    Self {
        records: LookupMap::new(b"records".to_vec()),
        ipfs: LookupMap::new(b"ipfs".to_vec()),
    }
  }
}

#[near_bindgen]
impl Welcome {
    // bye 
    pub fn set_bye(&mut self, message: String) {
        let signer_account_id = env::signer_account_id();   // initial
        let current_account_id = env::current_account_id(); // contract id
        let predecessor_account_id = env::predecessor_account_id(); // contract id

        self.records.insert(&signer_account_id.to_string(), &message);


        // Logs
        env::log_str(
            &json!({
                "method": "set_bye",
                "message": message,
                "signer_account_id": signer_account_id.to_string(),
                "current_account_id": current_account_id.to_string(),
                "predecessor_account_id": predecessor_account_id.to_string(), 

            })
            .to_string(),
        );
    }

 
    pub fn get_bye(&self, account_id: AccountId) -> String { 
        match self.records.get(&account_id.to_string()) {
            Some(bye) => bye,
            None => "Bye Bye".to_string(),
        }        
    }

    #[payable]
    pub fn get_external_greeting(&mut self, account_id: AccountId) -> Promise {
        // let signer_account_id = env::signer_account_id();   // initial
        env::log_str("incio la llamada");

        ext_contract_secondary::get_greeting( 
            account_id,
            AccountId::new_unchecked("dev-1652472334005-76834385663034".to_string()),
            0,
            CALLBACK_GAS
        ).then(
            
        
            ext_self::on_get_greeting(
                env::current_account_id(), //local 
                0,
                CALLBACK_GAS
            )
        ) 
    }

    #[private]
    pub fn on_get_greeting(&mut self)-> String{
        env::log_str("incio la on_get_greeting");

        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_val) => { 
                let result = promise_result_as_success(); 
                let res: String = near_sdk::serde_json::from_slice::<String>(&result.unwrap())
                                .expect("Unable to unwrap the result into a Pet");
                res

            },
            PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder}; 
    use near_sdk::{testing_env, VMContext};
    // use std::convert::TryInto;    

 
    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(accounts(0))
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_then_get_bye() {
        // let context = get_context(vec![], false);
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_bye("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_bye(accounts(0))
        );
    }

    #[test]
    fn get_default_bye() {
        // let context = get_context(vec![], true);
        let context = get_context(false);
        testing_env!(context);
        let contract = Welcome::default();
        // this test did not call set_bye so should return the default "Hello" bye
        assert_eq!(
            "Bye Bye".to_string(),
            contract.get_bye(accounts(1))
        );
    }
 

    // #[test]
    // fn set_then_get_external_greeting() {
    //     // let context = get_context(vec![], false);
    //     let context = get_context(false);
    //     testing_env!(context);
    //     let mut contract = Welcome::default();
    //     contract.set_bye("howdy".to_string());

    //     assert_eq!(
    //         "Hola Mundo".to_string(),
    //         contract.get_external_greeting(accounts(0))
    //     );
    // }

    

}
