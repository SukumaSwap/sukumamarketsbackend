use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env,
  serde::{Deserialize, Serialize},
  AccountId, Timestamp,
};

use crate::*;

// #[near_bindgen]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
  pub id: String,
  pub trade_type: String, // buy or sell
  pub seller: AccountId,
  pub buyer: AccountId,
  pub amount: u128,
  pub chat_id: String,  // Chat ID of the trade represented in firebase chat msgs
  pub token_id: String, // Token ID
  pub start_timestamp: Option<Timestamp>,
  pub end_timestamp: Option<Timestamp>,
}

// #[near_bindgen]
impl Trade {
  pub fn new(
    id: String,
    trade_type: String,
    seller: AccountId,
    buyer: AccountId,
    amount: u128,
    chat_id: String,
    token_id: String,
    start_timestamp: Option<Timestamp>,
    end_timestamp: Option<Timestamp>,
  ) -> Self {
    Self {
      id,
      trade_type,
      seller,
      buyer,
      amount,
      chat_id,
      token_id,
      start_timestamp,
      end_timestamp,
    }
  }

  pub fn update_end_timestamp(&mut self) {
    self.end_timestamp = Some(env::block_timestamp());
  }

  pub fn update_chat_id(&mut self, chat_id: String) {
    self.chat_id = chat_id;
  }

  pub fn update_amount(&mut self, amount: u128) {
    self.amount = amount;
  }
}

#[near_bindgen]
impl Contract {
  pub fn add_trade(
    &mut self,
    id: String,
    trade_type: String,
    seller: AccountId,
    buyer: AccountId,
    amount: u128,
    chat_id: String,
    token_id: String,
    start_timestamp: Option<Timestamp>,
    end_timestamp: Option<Timestamp>,
  ) {
    let trade = Trade::new(
      id,
      trade_type,
      seller,
      buyer,
      amount,
      chat_id,
      token_id,
      start_timestamp,
      end_timestamp,
    );
    self.trades.push(&trade);
  }

  pub fn get_trade(&self, id: String) -> Option<Trade> {
    let trades: &Vector<Trade> = &self.trades;
    let trade = trades.iter().find(|trad| trad.id == id);
    return trade;
  }

  pub fn get_all_trades(&self) -> Vec<Trade> {
    let mut trades = Vec::new();
    for trade in self.trades.iter() {
      trades.push(trade.clone());
    }
    trades
  }

  pub fn get_trades_by_account(&self, account_id: AccountId) -> Vec<Trade> {
    // let account_id = env::predecessor_account_id();
    let mut trades = Vec::new();
    for trade in self.trades.iter() {
      if trade.seller == account_id || trade.buyer == account_id {
        trades.push(trade.clone());
      }
    }
    trades
  }

  pub fn get_trades_length_by_account(&self, account_id: AccountId) -> usize {
    self.get_trades_by_account(account_id).len()
  }
}
