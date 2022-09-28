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
  pub amount: u32,
  pub date: Timestamp,
}


impl Revenue{

}

#[near_bindgen]
impl Contract{
    pub fn create_revenue(&mut self, from:String, amount: u32){
        let rev = Revenue { from, amount, date: env::block_timestamp() };
        self.revenues.insert(&rev);
    }
}