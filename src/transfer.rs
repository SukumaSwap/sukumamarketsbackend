use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env,
  serde::{Deserialize, Serialize},
  AccountId, Timestamp,
};

use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Transfer {
  pub id: String,
  pub sender: AccountId,
  pub receiver: AccountId,
  pub amount: u128,
  pub timestamp: Timestamp,
}

impl Transfer {
  pub fn new(id: String, sender: AccountId, receiver: AccountId, amount: u128) -> Self {
    Self {
      id,
      sender,
      receiver,
      amount,
      timestamp: env::block_timestamp(),
    }
  }
}

#[near_bindgen]
impl Contract {
  pub fn add_transfer(&mut self, id: String, sender: AccountId, receiver: AccountId, amount: u128) {
    let transfer = Transfer::new(id.clone(), sender, receiver, amount);
    self.transfers.push(&transfer);
  }

  pub fn get_transfers_len(&self) -> u64 {
    self.transfers.len()
  }

  // #[result_serializer(borsh)]
  pub fn get_transfer(&self, id: String) -> Option<Transfer> {
    for transfer in self.transfers.to_vec().iter() {
      if transfer.id == id {
        return Some(transfer.clone());
      }
    }
    None
  }

  pub fn get_account_transfers(&self, account_id: AccountId) -> Vec<Transfer> {
    self
      .transfers
      .iter()
      .filter(|transfer| {
        transfer.sender == account_id.clone() || transfer.receiver == account_id.clone()
      })
      .collect()
  }

  pub fn get_all_transfers(&self) -> Vec<Transfer> {
    let mut transfers = Vec::new();
    for transfer in self.transfers.to_vec().iter() {
      transfers.push(transfer.clone());
    }
    transfers
  }

  pub fn get_transfers_len_by_account(&self, account_id: AccountId) -> usize {
    self.get_account_transfers(account_id).len()
  }

}
