use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  json_types::U128,
  serde::{Deserialize, Serialize},
  AccountId, Timestamp,
};

use crate::{account::PubAccountInfo, *};

// #[near_bindgen]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenOffer {
  pub id: String,
  pub offer_type: String, // buy or sell
  pub offerer: AccountId,
  pub min_amount: u128,
  pub max_amount: u128,
  pub offer_rate: f32,
  pub active: bool,
  pub token: AccountId,
  pub payment: String,
  pub currency: String,
  pub instructions: String,
  pub created_on: Timestamp,
  pub updated_on: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CompleteTokenOffer {
  pub id: String,
  pub offer_type: String, // buy or sell
  pub offerer: Option<PubAccountInfo>,
  pub min_amount: u128,
  pub max_amount: u128,
  pub offer_rate: f32,
  pub active: bool,
  pub token: Option<TokenMetadata>,
  pub payment: Option<PaymentMethod>,
  pub currency: String,
  pub instructions: String,
  pub created_on: Timestamp,
  pub updated_on: Option<Timestamp>,
}

// #[near_bindgen]
impl TokenOffer {
  pub fn new(
    id: String,
    offer_type: String,
    offerer: AccountId,
    min_amount: U128,
    max_amount: U128,
    offer_rate: f32,
    token: AccountId,
    payment: String,
    currency: String,
    instructions: String,
  ) -> Self {
    Self {
      id,
      offer_type,
      offerer,
      min_amount: u128::from(min_amount),
      max_amount: u128::from(max_amount),
      offer_rate,
      active: true,
      token,
      payment,
      currency,
      instructions,
      created_on: env::block_timestamp(),
      updated_on: None,
    }
  }

  pub fn update_offer_rate(&mut self, offer_rate: f32) {
    self.offer_rate = offer_rate;
  }

  pub fn update_offer_status(&mut self, active: bool) {
    self.active = active;
  }
  pub fn make_complete_offer(&self, payment: Option<PaymentMethod>) -> CompleteTokenOffer {
    return CompleteTokenOffer {
      id: self.id.clone(),
      offer_type: self.offer_type.clone(),
      offerer: None,
      min_amount: self.min_amount.clone(),
      max_amount: self.max_amount,
      offer_rate: self.offer_rate,
      active: self.active,
      payment,
      currency: self.currency.clone(),
      instructions: self.instructions.clone(),
      token: None,
      created_on: self.created_on,
      updated_on: self.updated_on,
    };
  }
}

#[near_bindgen]
impl Contract {
  pub fn add_token_offer(
    &mut self,
    id: String,
    offer_type: String,
    offerer: AccountId,
    min_amount: U128,
    max_amount: U128,
    offer_rate: f32,
    token: AccountId,
    payment: String,
    currency: String,
    instructions: String,
  ) -> String {
    // Check offer type, if buy, don't check account balance
    let account = self.get_account(offerer.clone()).unwrap();
    let offer = TokenOffer::new(
      id.clone(),
      offer_type.clone(),
      offerer,
      min_amount,
      max_amount,
      offer_rate,
      token.clone(),
      payment,
      currency,
      instructions,
    );

    if offer_type.clone() == "buy".to_string() {
      self.tokenoffers.insert(&id.clone(), &offer);
      return "Offer created successfully".to_string();
    } else {
      if account.get_token_balance(token.clone()) >= offer.max_amount.clone() {
        self.tokenoffers.insert(&id.clone(), &offer);
        return "Offer created successfully".to_string();
      } else {
        // panic!("You do not have enough balance to add offer");
        return "You do not have enough balance to add offer".to_string();
      }
    }
  }

  pub fn get_account_token_offers(&self, account_id: AccountId) -> Vec<TokenOffer> {
    let mut offers = Vec::new();
    self
      .tokenoffers
      .to_vec()
      .into_iter()
      .for_each(|(_id, offer)| {
        if offer.offerer == account_id {
          offers.push(offer)
        }
      });
    offers
  }

  pub fn get_t_offers_len_by_account(&self, account_id: AccountId) -> usize {
    self.get_account_token_offers(account_id).len()
  }

  pub fn get_buy_token_offers(&self) -> Vec<CompleteTokenOffer> {
    let mut offers = Vec::new();
    self
      .tokenoffers
      .to_vec()
      .into_iter()
      .for_each(|(_id, offer)| {
        if offer.offer_type == "buy".to_string() {
          let payment = self.get_payment(offer.payment.clone());
          let mut comp_offer = offer.make_complete_offer(payment);
          let offerer = self.acc_pub_info(offer.offerer);
          let token_meta = self.get_token(offer.token);
          comp_offer.token = token_meta;
          comp_offer.offerer = offerer;
          offers.push(comp_offer)
        }
      });
    offers
  }

  pub fn get_buy_token_offers_by_token(&self, token: AccountId) -> Vec<CompleteTokenOffer> {
    let mut offers = Vec::new();
    self
      .tokenoffers
      .to_vec()
      .into_iter()
      .for_each(|(_id, offer)| {
        if offer.offer_type == "buy".to_string() && offer.token == token {
          let payment = self.get_payment(offer.payment.clone());
          let mut comp_offer = offer.make_complete_offer(payment);
          let offerer = self.acc_pub_info(offer.offerer);
          let token_meta = self.get_token(offer.token);
          comp_offer.token = token_meta;
          comp_offer.offerer = offerer;
          offers.push(comp_offer)
        }
      });
    offers
  }

  // Used internally
  pub fn get_token_offer(&self, offer_id: String) -> Option<TokenOffer> {
    self.tokenoffers.get(&offer_id)
  }

  // Used externally from the frontend
  pub fn pub_get_token_offer(&self, offer_id: String) -> Option<CompleteTokenOffer> {
    let offer = self.tokenoffers.get(&offer_id).unwrap();
    let payment = self.get_payment(offer.payment.clone());
    let mut comp_offer = offer.make_complete_offer(payment);
    let offerer = self.acc_pub_info(offer.offerer);
    let token_meta = self.get_token(offer.token);
    comp_offer.token = token_meta;
    comp_offer.offerer = offerer;
    Some(comp_offer)
  }

  pub fn get_sell_token_offers(&self) -> Vec<CompleteTokenOffer> {
    let mut offers = Vec::new();
    self
      .tokenoffers
      .to_vec()
      .into_iter()
      .for_each(|(_id, offer)| {
        if offer.offer_type == "sell".to_string() {
          let payment = self.get_payment(offer.payment.clone());
          let mut comp_offer = offer.make_complete_offer(payment);
          let offerer = self.acc_pub_info(offer.offerer);
          let token_meta = self.get_token(offer.token);
          comp_offer.token = token_meta;
          comp_offer.offerer = offerer;
          offers.push(comp_offer)
        }
      });
    offers
  }

  pub fn get_sell_token_offers_by_token(&self, token: AccountId) -> Vec<CompleteTokenOffer> {
    let mut offers = Vec::new();
    self
      .tokenoffers
      .to_vec()
      .into_iter()
      .for_each(|(_id, offer)| {
        if offer.offer_type == "sell".to_string() && offer.token == token {
          let payment = self.get_payment(offer.payment.clone());
          let mut comp_offer = offer.make_complete_offer(payment);
          let offerer = self.acc_pub_info(offer.offerer);
          let token_meta = self.get_token(offer.token);
          comp_offer.token = token_meta;
          comp_offer.offerer = offerer;
          offers.push(comp_offer)
        }
      });
    offers
  }

  pub fn get_all_token_offers(&self) -> Vec<TokenOffer> {
    let mut offers = Vec::new();
    self
      .tokenoffers
      .to_vec()
      .into_iter()
      .for_each(|(_id, offer)| offers.push(offer));
    offers
  }

  pub fn update_token_offer_status(&mut self, offer_id: String, active: bool) {
    let mut offer = self.tokenoffers.remove(&offer_id.clone()).unwrap();
    offer.update_offer_status(active);
    self.tokenoffers.insert(&offer_id, &offer);
  }

  pub fn clear_token_offers(&mut self) {
    self.tokenoffers.clear();
  }
}
