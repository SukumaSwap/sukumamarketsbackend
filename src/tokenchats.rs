use core::panic;

use near_sdk::json_types::U128;
use near_sdk::{
  borsh::{BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
  AccountId, Balance, Timestamp,
};


use crate::tokenoffers::CompleteTokenOffer;
use crate::*;

// #[near_bindgen]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenChat {
  pub id: String,
  pub offer_id: String,
  pub token_id: AccountId,
  pub owner: AccountId,
  pub offerer: AccountId,
  pub amount: Balance,
  pub trade_cost: Balance,
  pub trade_cost_usd: f64,
  pub started_at: Timestamp,
  pub ended_at: Option<Timestamp>,
  pub active: bool,
  pub payer: AccountId,
  pub receiver: AccountId,
  pub paid: bool,
  pub received: bool,
  pub canceled: bool,
  pub released: bool,
  pub payer_has_rated: bool,
  pub receiver_has_rated: bool,
  pub payment_msg: String,
  pub created_on: Timestamp,
  pub updated_on: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CompleteTokenChat {
  chat: Option<TokenChat>,
  offer: Option<CompleteTokenOffer>,
}

// #[near_bindgen]
impl TokenChat {
  pub fn new(
    id: String,
    offer_id: String,
    token_id: AccountId,
    owner: AccountId,
    offerer: AccountId,
    amount: u128,
    trade_cost: u128,
    trade_cost_usd: f64,
    payer: AccountId,
    receiver: AccountId,
    payment_msg: String,
  ) -> Self {
    Self {
      id,
      offer_id,
      token_id,
      owner,
      offerer,
      amount,
      trade_cost,
      trade_cost_usd,
      started_at: env::block_timestamp(),
      ended_at: None,
      active: true,
      payer,
      receiver,
      paid: false,
      received: false,
      canceled: false,
      released: false,
      payer_has_rated: false,
      receiver_has_rated: false,
      payment_msg,
      created_on: env::block_timestamp(),
      updated_on: Some(env::block_timestamp()),
    }
  }

  pub fn update_ended_at(&mut self) {
    self.ended_at = Some(env::block_timestamp());
  }

  pub fn update_active_status(&mut self, active: bool) {
    self.active = active;
  }

  pub fn mark_as_paid(&mut self) -> String {
    let payer = env::predecessor_account_id();
    if self.payer == payer {
      self.paid = true;
      return "success".to_string();
    }
    "failed".to_string()
  }

  pub fn mark_as_received(&mut self) -> String {
    let receiver = env::predecessor_account_id();
    if self.receiver == receiver {
      self.received = true;
      return "success".to_string();
    }
    "failed".to_string()
  }

  pub fn mark_as_released(&mut self) {
    self.released = true;
  }

  pub fn mark_as_canceled(&mut self) -> String {
    if self.paid == false && self.received == false {
      self.canceled = true;
      self.active = false;
      return "chat canceled".to_string();
    } else {
      return "Chat cannot be cancelled since its either marked as paid or received".to_string();
    }
  }
}

#[near_bindgen]
impl Contract {
  pub fn add_token_buy_chat(
    &mut self,
    id: String,
    offer_id: String,
    token_id: AccountId,
    owner: AccountId,
    amount: U128,
    payer: AccountId,
    receiver: AccountId,
    payment_msg: String,
    trade_cost: U128,
    trade_cost_usd: f64
  ) -> String {
    let offer = self.get_token_offer(offer_id.clone());

    if owner.clone() == offer.as_ref().unwrap().offerer.clone() {
      // panic!("You can't chat with yourself");
      return "You can't chat with yourself".to_string();
    }
    if offer.as_ref().is_none() {
      return "Offer not found".to_string();
    } else {
      if offer.as_ref().unwrap().offer_type.clone() == "buy".to_string() {
        let chat_initiator = self.get_account(payer.clone());
        if chat_initiator.as_ref().is_none() {
          return "You must be a registered user to chat with someone".to_string();
        } else if chat_initiator.as_ref().unwrap().clone().balance < u128::from(trade_cost.clone()) {
          return "You don't have enough balance to chat with someone".to_string();
        } else {
          if chat_initiator
            .as_ref()
            .unwrap()
            .clone()
            .get_token_balance(token_id.clone())
            < u128::from(amount.clone())
          {
            return "You don't have enough balance to chat with someone".to_string();
          } else {
            self.tokenchats.insert(
              &id.clone(),
              &TokenChat::new(
                id.clone(),
                offer_id.clone(),
                token_id.clone(),
                owner.clone(),
                offer.as_ref().unwrap().offerer.clone(),
                u128::from(amount.clone()),
                u128::from(trade_cost.clone()),
                trade_cost_usd.clone(),
                payer.clone(),
                receiver.clone(),
                payment_msg,
              ),
            );
            self
              .get_account(payer.clone())
              .unwrap()
              .lock_tokens(token_id.clone(), u128::from(amount.clone()));
            self
              .get_account(payer.clone())
              .unwrap()
              .lock(u128::from(trade_cost.clone()));
            return "created".to_string();
          }
        }
      }
      return "Offer is not for buy".to_string();
    }
  }
 
  pub fn add_token_sell_chat(
    &mut self,
    id: String,
    offer_id: String,
    token_id: AccountId,
    owner: AccountId,
    amount: U128,
    payer: AccountId,
    receiver: AccountId,
    payment_msg: String,
    trade_cost: U128,
    trade_cost_usd: f64
  ) -> String {
    let offer = self.get_token_offer(offer_id.clone());

    if owner.clone() == offer.as_ref().unwrap().offerer.clone() {
      return "You can't chat with yourself".to_string();
    }
    if offer.as_ref().is_none() {
      return "Offer not found".to_string();
    } else {
      if offer.as_ref().unwrap().offer_type.clone() == "sell".to_string() {
        let offer_owner = self.get_account(receiver.clone());
        if offer_owner
          .as_ref()
          .unwrap()
          .get_token_balance(token_id.clone())
          < u128::from(amount.clone())
        {
          return "Offerer does not have sufficient balance to hold the trade.".to_string();
        } else if offer_owner.as_ref().unwrap().balance < u128::from(trade_cost.clone()) {
          return "Offerer does not have sufficient balance to hold the trade. ".to_string();
        } else {
          self.tokenchats.insert(
            &id.clone(),
            &TokenChat::new(
              id,
              offer_id,
              token_id.clone(),
              owner,
              offer.as_ref().unwrap().offerer.clone(),
              u128::from(amount.clone()),
              u128::from(trade_cost.clone()),
              trade_cost_usd.clone(),
              payer.clone(),
              receiver.clone(),
              payment_msg,
            ),
          );
          self
            .get_account(receiver.clone())
            .unwrap()
            .lock_tokens(token_id.clone(), u128::from(amount.clone()));
          self
            .get_account(receiver.clone())
            .unwrap()
            .lock(u128::from(trade_cost.clone()));
          return "created".to_string();
        }
      }
      return "Offer is not for sell".to_string();
    }
  }

  pub fn get_token_chat(&mut self, chat_id: String) -> Option<TokenChat> {
    self.tokenchats.get(&chat_id)
  }

  pub fn pub_get_token_chat(&mut self, chat_id: String) -> CompleteTokenChat {
    let chat = self.tokenchats.get(&chat_id);
    let offer = self.pub_get_token_offer(chat.as_ref().unwrap().offer_id.clone());
    CompleteTokenChat { chat, offer }
  }

  pub fn mark_token_as_paid(&mut self, chat_id: String) -> String {
    let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();

    if chat.clone().active {
      chat.mark_as_paid();
      self.tokenchats.insert(&chat_id.clone(), &chat.clone());
      return "success".to_string();
    } else {
      self.tokenchats.insert(&chat_id.clone(), &chat.clone());
      return "chat is not active".to_string();
    }
  }

  pub fn mark_token_as_received(&mut self, chat_id: String) -> String {
    let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();

    if chat.clone().active {
      chat.mark_as_received();
      self.tokenchats.insert(&chat_id.clone(), &chat.clone());
      self.release_tokens(chat_id.clone());
      return "success".to_string();
    } else {
      self.tokenchats.insert(&chat_id.clone(), &chat.clone());
      return "chat is not active".to_string();
    }
  }

  pub fn remove_token_chat(&mut self) {
    self.assert_owner();
  }

  pub fn get_token_account_chats(&self, account_id: AccountId) -> Vec<TokenChat> {
    // self.assert_account_owner(account_id.clone());

    let mut tokenchats = Vec::new();
    self
      .tokenchats
      .to_vec()
      .into_iter()
      .for_each(|(_id, chat)| {
        if chat.owner == account_id || chat.offerer == account_id {
          tokenchats.push(chat)
        }
      });
    tokenchats
  }

  pub fn clear_token_chats(&mut self) {
    self.tokenchats.clear();
  }

  pub fn cancel_token_chat(&mut self, chat_id: String) -> String {
    let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();
    let offer = self.get_token_offer(chat.clone().offer_id).unwrap();

    if chat.clone().active {
      if offer.offer_type == "buy" {
        self
          .get_account(chat.clone().owner)
          .unwrap()
          .unlock_tokens(chat.clone().token_id, u128::from(chat.clone().amount));
      } else {
        self
          .get_account(offer.offerer.clone())
          .unwrap()
          .unlock_tokens(chat.clone().token_id, u128::from(chat.clone().amount));
      }
      chat.mark_as_canceled();
      self.tokenchats.insert(&chat_id.clone(), &chat);
      return "chat canceled".to_string();
    } else {
      self.tokenchats.insert(&chat_id.clone(), &chat.clone());
      return "chat is not active".to_string();
    }
  }

  pub fn release_tokens(&mut self, chat_id: String) {
    let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();
    let offer = self.get_token_offer(chat.offer_id.clone()).unwrap();

    if offer.offer_type.clone() == "buy".to_string() {
      if chat.clone().released {
        self.tokenchats.insert(&chat_id.clone(), &chat.clone());
        panic!("Amount already released");
      } else {
        chat.mark_as_released();
        self.tokenchats.insert(&chat_id.clone(), &chat.clone());
        // self.trades.push(&trade);
        self.create_revenue(
          chat.clone().token_id.clone().to_string(),
          "trade".to_string(),
          chat.clone().payer.clone(),
          chat.clone().trade_cost.clone(),
          chat.clone().trade_cost_usd.clone(),
        );
        self.send_tokens(
          chat.clone().payer.clone(),
          chat.clone().receiver.clone(),
          offer.token,
          U128::from(chat.clone().amount),
          chat_id.clone(),
        );
      }
    } else {
      if chat.clone().released {
        self.tokenchats.insert(&chat_id.clone(), &chat.clone());
        panic!("Amount already released");
      } else {
        chat.mark_as_released();
        chat.update_ended_at();
        self.tokenchats.insert(&chat_id.clone(), &chat.clone());
        // self.trades.push(&trade);
        self.create_revenue(
          chat.clone().token_id.clone().to_string(),
          "trade".to_string(),
          chat.clone().receiver.clone(),
          chat.clone().trade_cost.clone(),
          chat.clone().trade_cost_usd.clone(),
        );
        self.send_tokens(
          chat.clone().receiver.clone(),
          chat.clone().payer.clone(),
          offer.token,
          U128::from(chat.clone().amount),
          chat_id.clone(),
        );
      }
    }
  }

  pub fn receiver_token_rate_chat(&mut self, chat_id: String, rating: bool) {
    let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();
    let account_id = env::predecessor_account_id();
    if account_id.clone() == chat.clone().receiver {
      if chat.clone().receiver_has_rated {
        if rating.clone() {
          self
            .get_account(chat.clone().payer.clone())
            .unwrap()
            .add_like();
        } else {
          self
            .get_account(chat.clone().payer.clone())
            .unwrap()
            .add_dislike();
        }
      }
    }
    chat.receiver_has_rated = true;
    self.tokenchats.insert(&chat_id.clone(), &chat);
  }

  pub fn payer_token_rate_chat(&mut self, chat_id: String, rating: bool) {
    let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();
    let account_id = env::predecessor_account_id();
    if account_id.clone() == chat.clone().payer {
      if chat.clone().payer_has_rated {
        if rating.clone() {
          self
            .get_account(chat.clone().receiver.clone())
            .unwrap()
            .add_like();
        } else {
          self
            .get_account(chat.clone().receiver.clone())
            .unwrap()
            .add_dislike();
        }
      }
    }
    chat.payer_has_rated = true;
    self.tokenchats.insert(&chat_id.clone(), &chat.clone());
  }
}
