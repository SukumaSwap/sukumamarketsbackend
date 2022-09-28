use near_sdk::json_types::U128;
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
  AccountId,
};

use crate::account::PubAccountInfo;
use crate::*;

// #[near_bindgen]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
  pub id: String,
  pub offer_type: String, // buy or sell
  pub offerer: AccountId,
  pub min_amount: u128,
  pub max_amount: u128,
  pub offer_rate: f32,
  pub active: bool,
  pub payment: String,
  pub currency: String,
  pub instructions: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CompleteOffer {
  pub id: String,
  pub offer_type: String, // buy or sell
  pub offerer: Option<PubAccountInfo>,
  pub min_amount: u128,
  pub max_amount: u128,
  pub offer_rate: f32,
  pub active: bool,
  pub payment: Option<PaymentMethod>,
  pub currency: String,
  pub instructions: String,
}

// #[near_bindgen]
impl Offer {
  pub fn new(
    id: String,
    offer_type: String,
    offerer: AccountId,
    min_amount: U128,
    max_amount: U128,
    offer_rate: f32,
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
      payment,
      currency,
      instructions,
      active: true,
    }
  }

  pub fn update_offer_rate(&mut self, offer_rate: f32) {
    self.offer_rate = offer_rate;
  }

  pub fn update_offer_status(&mut self, active: bool) {
    self.active = active;
  }

  pub fn make_complete_offer(&self, payment: Option<PaymentMethod>) -> CompleteOffer {
    return CompleteOffer {
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
    };
  }
}

#[near_bindgen]
impl Contract {
  pub fn add_offer(
    &mut self,
    id: String,
    offer_type: String,
    offerer: AccountId,
    min_amount: U128,
    max_amount: U128,
    offer_rate: f32,
    payment: String,
    currency: String,
    instructions: String,
  ) -> String {
    // Check offer type, if buy, don't check account balance
    let account = self.get_account(offerer.clone()).unwrap();
    let offer = Offer::new(
      id.clone(),
      offer_type.clone(),
      offerer,
      min_amount,
      max_amount,
      offer_rate,
      payment,
      currency,
      instructions,
    );

    if offer_type.clone() == "buy".to_string() {
      self.offers.insert(&id, &offer);
      return "Offer created successfully".to_string();
    } else {
      if account.balance >= offer.max_amount.clone() {
        self.offers.insert(&id, &offer);
        return "Offer created successfully".to_string();
      } else {
        // panic!("You do not have enough balance to add offer");
        return "You do not have enough balance to add offer".to_string();
      }
    }
  }

  pub fn get_account_offers(&self, account_id: AccountId) -> Vec<Offer> {
    let mut offers = Vec::new();
    self.offers.to_vec().into_iter().for_each(|(_id, offer)| {
      if offer.offerer == account_id {
        offers.push(offer)
      }
    });
    offers
  }

  pub fn get_offers_len_by_account(&self, account_id: AccountId) -> usize {
    self.get_account_offers(account_id).len()
  }
  // Used internally
  pub fn get_offer(&self, offer_id: String) -> Option<Offer> {
    self.offers.get(&offer_id)
  }

  // Used externally from the frontend
  pub fn pub_get_offer(&self, offer_id: String) -> Option<CompleteOffer> {
    let offer = self.offers.get(&offer_id).unwrap();
    let payment = self.get_payment(offer.payment.clone());
    let mut comp_offer = offer.make_complete_offer(payment);
    let offerer = self.acc_pub_info(offer.offerer);
    comp_offer.offerer = offerer;
    Some(comp_offer)
  }

  pub fn get_buy_offers(&self) -> Vec<CompleteOffer> {
    let mut offers = Vec::new();
    self.offers.to_vec().into_iter().for_each(|(_id, offer)| {
      if offer.offer_type == "buy".to_string() {
        let payment = self.get_payment(offer.payment.clone());
        let mut comp_offer = offer.make_complete_offer(payment);
        let offerer = self.acc_pub_info(offer.offerer);
        comp_offer.offerer = offerer;
        offers.push(comp_offer);
      }
    });
    offers
  }

  pub fn get_sell_offers(&self) -> Vec<CompleteOffer> {
    let mut offers = Vec::new();
    self.offers.to_vec().into_iter().for_each(|(_id, offer)| {
      if offer.offer_type == "sell".to_string() {
        let payment = self.get_payment(offer.payment.clone());
        let mut comp_offer = offer.make_complete_offer(payment);
        let offerer = self.acc_pub_info(offer.offerer);
        comp_offer.offerer = offerer;
        offers.push(comp_offer);
      }
    });
    offers
  }

  pub fn update_offer_status(&mut self, offer_id: String, active: bool) {
    let mut offer = self.offers.remove(&offer_id.clone()).unwrap();
    offer.update_offer_status(active);
    self.offers.insert(&offer_id, &offer);
  }

  pub fn clear_offers(&mut self) {
    self.offers.clear();
  }
}
