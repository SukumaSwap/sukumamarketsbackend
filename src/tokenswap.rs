use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::Timestamp;

use crate::*;

// #[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenSwap {
  pub id: String,
  pub from_token: String,
  pub to_token: String,
  pub from_token_amount: u64,
  pub timestamp: Timestamp,
}

impl TokenSwap {
  pub fn new(id: String, from_token: String, to_token: String, from_token_amount: u64) -> Self {
    Self {
      id,
      from_token,
      to_token,
      from_token_amount,
      timestamp: Timestamp::default(),
    }
  }
}

#[near_bindgen]
impl Contract {
}
