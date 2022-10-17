use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  json_types::U128,
};

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Revenue {
  pub from: String, // Trade or transfer
  pub amount: u128,
  pub date: Timestamp,
}

impl Revenue {}

#[near_bindgen]
impl Contract {
  pub fn create_revenue(&mut self, from: String, account: AccountId, amount: U128) {

    self
    .get_account(account.clone())
    .unwrap().unlock(u128::from(amount.clone()));

    self
    .get_account(account.clone())
    .unwrap().withdraw(u128::from(amount.clone()));

    let amt = u128::from(amount.clone());
    let rev = Revenue {
      from,
      amount: amt.clone(),
      date: env::block_timestamp(),
    };
    self.revenue += amt;
    self.revenues.insert(&rev);
  }
}
