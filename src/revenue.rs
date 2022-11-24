use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
};

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Revenue {
  pub from: String, // Trade or transfer
  pub asset: String, // Near, specific token
  pub amount_usd: f64,
  pub amount: u128,
  pub date: Timestamp,
}

impl Revenue {}

#[near_bindgen]
impl Contract {
  pub fn create_revenue(&mut self, asset: String, from: String, account: AccountId, amount: u128, amount_usd: f64 ) {

    self
    .get_account(account.clone())
    .unwrap().unlock(amount.clone());

    self
    .get_account(account.clone())
    .unwrap().withdraw(amount.clone());

    let rev = Revenue {
      asset,
      from,
      amount: amount.clone(),
      amount_usd,
      date: env::block_timestamp(),
    };
    self.revenue += amount.clone();
    self.revenue_usd += amount_usd.clone();
    self.revenues.insert(&rev);
  }
}
