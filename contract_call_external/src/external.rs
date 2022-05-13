// use crate::*;
use near_sdk::{ext_contract}; // used to handle remote contract invocation

// External
#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn on_get_greeting();
}
#[ext_contract(ext_contract_secondary)]
trait ExtContract {
    fn get_greeting(&self, account_id: AccountId);
}

// ----------------------------------------------