use crate::account::Account;
use crate::chat::Chat;
use crate::errors::*;
use crate::offer::Offer;
use crate::tokenswap::TokenSwap;
use crate::trade::Trade;
use crate::transfer::Transfer;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, Timestamp};
use revenue::Revenue;
use std::collections::HashMap;
use tokenchats::TokenChat;
use tokenoffers::TokenOffer;
pub mod account;
pub mod chat;
pub mod constants;
pub mod errors;
pub mod fungibletoken;
pub mod offer;
pub mod owner;
pub mod revenue;
pub mod tests;
pub mod tokenchats;
pub mod tokenoffers;
pub mod tokenswap;
pub mod trade;
pub mod transfer;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
  Transfers,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
  pub address: AccountId,
  pub name: String,
  pub symbol: String,
  pub icon: String,
  pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethod {
  pub name: String,
  pub icon: String,
}


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contract {
  pub owner_id: AccountId,
  pub proposed_owner_id: AccountId,
  pub guardians: UnorderedSet<AccountId>,
  pub accounts: HashMap<AccountId, Account>,
  pub trades: Vector<Trade>,
  pub transfers: Vector<Transfer>,
  pub tokenswaps: LookupMap<String, TokenSwap>,
  pub offers: UnorderedMap<String, Offer>,
  pub tokenoffers: UnorderedMap<String, TokenOffer>,
  pub chats: UnorderedMap<String, Chat>,
  pub tokenchats: UnorderedMap<String, TokenChat>,
  pub transfer_cost: u32,
  pub send_cost: f32,
  pub tokens: UnorderedMap<AccountId, TokenMetadata>,
  pub whitelistedtokens: UnorderedMap<AccountId, TokenMetadata>,
  pub payment_methods: UnorderedMap<String, PaymentMethod>,
  pub revenue: u128,
  pub revenue_usd: f64,
  pub revenues: UnorderedSet<Revenue>,
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      owner_id: env::current_account_id(),
      proposed_owner_id: env::current_account_id(),
      trades: Vector::new(b"a".to_vec()),
      transfers: Vector::new(StorageKey::Transfers),
      tokenswaps: LookupMap::new(b"b".to_vec()),
      transfer_cost: 0,
      send_cost: 0.05,
      accounts: HashMap::new(),
      offers: UnorderedMap::new(b"c".to_vec()),
      tokenoffers: UnorderedMap::new(b"d".to_vec()),
      chats: UnorderedMap::new(b"e".to_vec()),
      tokenchats: UnorderedMap::new(b"f".to_vec()),
      guardians: UnorderedSet::new(b"g".to_vec()),
      tokens: UnorderedMap::new(b"h".to_vec()),
      whitelistedtokens: UnorderedMap::new(b"k".to_vec()),
      payment_methods: UnorderedMap::new(b"i".to_vec()),
      revenue: 0,
      revenue_usd: 0.0,
      revenues: UnorderedSet::new(b"l".to_vec()),
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      owner_id: env::current_account_id(),
      proposed_owner_id: env::current_account_id(),
      trades: Vector::new(b"a".to_vec()),
      transfers: Vector::new(StorageKey::Transfers),
      tokenswaps: LookupMap::new(b"b".to_vec()),
      transfer_cost: 0,
      send_cost: 0.05,
      accounts: HashMap::new(),
      offers: UnorderedMap::new(b"c".to_vec()),
      tokenoffers: UnorderedMap::new(b"d".to_vec()),
      chats: UnorderedMap::new(b"e".to_vec()),
      tokenchats: UnorderedMap::new(b"f".to_vec()),
      guardians: UnorderedSet::new(b"g".to_vec()),
      tokens: UnorderedMap::new(b"h".to_vec()),
      whitelistedtokens: UnorderedMap::new(b"k".to_vec()),
      payment_methods: UnorderedMap::new(b"i".to_vec()),
      revenue: 0,
      revenue_usd: 0.0,
      revenues: UnorderedSet::new(b"l".to_vec()),
    }
  }

  pub fn add_payment_method(&mut self, method: PaymentMethod) {
    self.payment_methods.insert(&method.name, &method);
  }

  pub fn remove_payment_method(&mut self, method_name: String) {
    self.payment_methods.remove(&method_name);
  }

  pub fn add_token(&mut self, token: AccountId, metadata: TokenMetadata) {
    self.tokens.insert(&token, &metadata);
  }

  pub fn remove_token(&mut self, token: AccountId) {
    self.tokens.remove(&token);
  }

  pub fn add_whitelisted_token(&mut self, token: AccountId, metadata: TokenMetadata) {
    self.whitelistedtokens.insert(&token, &metadata);
  }

  pub fn remove_whitelisted_token(&mut self, token: AccountId) {
    self.whitelistedtokens.remove(&token);
  }

  pub fn update_token(&mut self, token: AccountId, metadata: TokenMetadata) {
    self.tokens.remove(&token.clone());
    self.tokens.insert(&token, &metadata);
  }

  pub fn get_tokens(&self) -> Vec<TokenMetadata> {
    self.tokens.values().collect()
  }

  pub fn get_token(&self, token: AccountId) -> Option<TokenMetadata> {
    self.tokens.get(&token)
  }

  pub fn get_send_cost(&self) -> f32 {
    return self.send_cost;
  }

  pub fn update_send_cost(&mut self, cost: f32){
    self.send_cost = cost;
  }

  pub fn get_payments(&self) -> Vec<PaymentMethod> {
    let mut methods = Vec::new();
    self
      .payment_methods
      .to_vec()
      .into_iter()
      .for_each(|(_id, method)| methods.push(method));
    methods
  }

  pub fn get_payment(&self, name: String) -> Option<PaymentMethod> {
    self.payment_methods.get(&name)
  }
}
