use core::panic;
use near_sdk::json_types::U128;
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, Balance, Promise, ONE_NEAR,
};
use std::collections::HashMap;

// use crate::constants::*;
use crate::*;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenBalanceInfo {
  pub token: Option<TokenMetadata>,
  pub balance: Balance,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
  pub id: AccountId,
  pub balance: Balance,
  pub locked: Balance,
  pub likes: i32,
  pub dislikes: i32,
  pub blocked_by: i32,
  pub created_on: Timestamp,
  pub tokens: Vec<TokenBalanceInfo>,
  pub locked_tokens: Vec<TokenBalanceInfo>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PubAccountInfo {
  pub id: AccountId,
  pub likes: i32,
  pub trades: usize,
  pub transfers: usize,
  pub offers: usize,
  pub dislikes: i32,
  pub blocked_by: i32,
  pub created_on: Timestamp,
}

// #[near_bindgen]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
  // Account identifier, matches the account ID
  pub id: AccountId,

  // Balance of the account
  pub balance: Balance,

  // Amount locked for an ongoing trade chat
  pub locked: Balance,

  // Likes of the account
  pub likes: i32,

  // Dislikes of the account
  pub dislikes: i32,
  // Tokens
  pub tokens: HashMap<AccountId, Balance>,

  // Locked tokens
  pub locked_tokens: HashMap<AccountId, Balance>,

  pub blocked_by: i32,
  pub created_on: Timestamp,
}

// imp for account and new, check whether the account is new or not
// #[near_bindgen]
impl Account {
  pub fn new(id: AccountId) -> Account {
    Account {
      id,
      balance: 0,
      locked: 0,
      likes: 0,
      dislikes: 0,
      tokens: HashMap::new(),
      locked_tokens: HashMap::new(),
      blocked_by: 0,
      created_on: env::block_timestamp(),
    }
  }

  pub fn deposit(&mut self, amount: u128) {
    assert!(ONE_NEAR <= amount, "Amount must be greater than 1 Near");
    self.balance += amount;
  }

  pub fn withdraw(&mut self, amount: u128) {
    assert!(
      self.balance >= amount,
      "Insufficient balance to make a withdrawal"
    );
    assert!(
      env::predecessor_account_id() == self.id,
      "Only the owner can withdraw"
    );
    self.balance -= amount;
  }

  pub fn deposit_tokens(&mut self, token_id: AccountId, amount: u128) {
    // assert!(ONE_NEAR <= amount, "Amount must be greater than 1 Near");
    self.tokens.entry(token_id.clone()).or_insert(0);
    self
      .tokens
      .insert(token_id.clone(), self.tokens[&token_id.clone()] + amount);
    // self.likes += 1;
    env::log_str("I have been reached");
  }

  pub fn withdraw_tokens(&mut self, token_id: AccountId, amount: u128) {
    let token = self.tokens.get_mut(&token_id);
    if token.is_none() {
      env::log_str("Token not found");
      return;
    }
    let token = token.unwrap();
    if *token < amount {
      env::log_str("Insufficient balance to make a withdrawal");
      return;
    }
    *token -= amount;
  }

  pub fn lock(&mut self, amount: u128) {
    assert!(
      self.balance >= amount,
      "Not enough balance to lock, can't proceed to lock the amount of {} for a trade.",
      amount
    );
    self.balance -= amount;
    self.locked += amount;
  }

  pub fn unlock(&mut self, amount: u128) {
    assert!(self.locked >= amount);
    self.locked -= amount.clone();
    self.balance += amount.clone();
  }

  pub fn lock_tokens(&mut self, token_id: AccountId, amount: u128) {
    // Check whether the token balance is greater than the amount to be locked from the tokens
    let token = self.tokens.get_mut(&token_id);
    if token.is_none() {
      panic!("Token not found");
    }
    let token = token.unwrap();
    if *token < amount.clone() {
      panic!("Insufficient token balance to lock");
    }
    // Lock the tokens
    self.locked_tokens.entry(token_id.clone()).or_insert(0);
    self.locked_tokens.insert(
      token_id.clone(),
      self.locked_tokens[&token_id.clone()] + amount.clone(),
    );
    // Subtract the locked tokens from the token balance in tokens
    *token -= amount.clone();
  }

  pub fn unlock_tokens(&mut self, token_id: AccountId, amount: u128) {
    let token = self.locked_tokens.get_mut(&token_id);
    if token.is_none() {
      panic!("Token not found");
    }
    let token = token.unwrap();
    if *token < amount {
      panic!("The token balance you are unlocking is more than the locked balance");
    }
    *token -= amount;
    // Add the token balance to the account tokens token balance
    let mytoken = self.tokens.get_mut(&token_id.clone());
    if mytoken.is_none() {
      panic!("Token not found");
    }
    let mytoken = mytoken.unwrap();
    *mytoken += amount;
  }

  pub fn get_token_balance(&self, token_id: AccountId) -> Balance {
    let token = self.tokens.get(&token_id);
    if token.is_none() {
      panic!("Token not found");
    }
    let token = token.unwrap();
    *token
  }

  pub fn release(&mut self, amount: u128, to: AccountId) -> Promise {
    if self.locked < amount.clone() {
      panic!("Amount to release is greater than locked balance, can't proceed with transaction");
    }
    self.unlock(amount.clone());
    self.withdraw(amount.clone());
    Promise::new(to).transfer(amount.clone())
  }

  pub fn token_release(&mut self, token_id: AccountId, amount: u128) {
    let locked_tokens = self.locked_tokens.get(&token_id);
    if locked_tokens.is_none() {
      panic!("Token not found");
    }
    let locked_tokens = locked_tokens.unwrap();
    if *locked_tokens < amount {
      panic!("Amount to release is greater than locked balance, can't proceed with transaction");
    }
    self.unlock_tokens(token_id.clone(), amount.clone());
    self.withdraw_tokens(token_id.clone(), amount.clone());
  }

  pub fn add_like(&mut self) {
    self.likes += 1;
  }

  pub fn remove_like(&mut self) {
    self.likes -= 1;
  }

  pub fn add_dislike(&mut self) {
    self.dislikes += 1;
  }

  pub fn remove_dislike(&mut self) {
    self.dislikes -= 1;
  }

  pub fn get_likes(&self) -> i32 {
    self.likes
  }

  pub fn get_dislikes(&self) -> i32 {
    self.dislikes
  }

  pub fn get_balance_as_string(&self) -> String {
    format!(
      "{:?} yoctoNears, approximately {:?} Near",
      self.balance,
      self.balance / ONE_NEAR
    )
  }

}

#[near_bindgen]
impl Contract {
  pub fn register_new_account(&mut self, account_id: AccountId) -> String {
    if self.get_account(account_id.clone()).is_none() {
      self
        .accounts
        .insert(account_id.clone(), Account::new(account_id.clone()));
      return "Account registered successfully".to_string();
    } else {
      return "Account already registered".to_string();
    };
  }

  pub fn get_account(&mut self, account_id: AccountId) -> Option<&mut Account> {
    self.accounts.get_mut(&account_id)
  }

  pub fn pub_get_account(&self, account_id: AccountId) -> Option<&Account> {
    self.accounts.get(&account_id)
  }

  #[payable]
  pub fn contract_deposit(&mut self, account_id: &AccountId) {
    assert!(
      env::attached_deposit() > ONE_NEAR,
      "Deposit must be greater than 1 Near"
    );

    let acc = match self.get_account(account_id.clone()) {
      Some(it) => it,
      _ => return,
    };
    acc.deposit(env::attached_deposit());
    self.add_transfer(
      env::block_timestamp().to_string(),
      account_id.clone(),
      env::current_account_id(),
      env::attached_deposit(),
    )
  }

  #[payable]
  pub fn withdraw_near(&mut self, amount: U128) -> Promise {
    let account_id = env::predecessor_account_id();
    let acc = self.get_account(account_id.clone());
    if acc.is_none() {
      panic!("Account not found");
    }
    assert!(
      acc.as_ref().unwrap().clone().balance >= u128::from(amount.clone()),
      "Amount to withdraw is greater than balance, can't proceed with transaction"
    );
    acc.unwrap().withdraw(u128::from(amount.clone()));
    self.add_transfer(
      env::block_timestamp().to_string(),
      env::current_account_id(),
      account_id.clone(),
      u128::from(amount),
    );
    Promise::new(account_id.clone()).transfer(u128::from(amount.clone()))
  }

  pub fn get_all_accounts(&mut self) -> Vec<&Account> {
    assert!(
      env::predecessor_account_id() == env::current_account_id(),
      "Only contract owner can call this method"
    );
    let input = self.accounts.values();
    let mut output = Vec::new();

    for s in input {
      output.push(s);
    }

    output
  }

  pub fn get_account_balance_as_string(&mut self, account_id: &AccountId) -> String {
    let acc = match self.get_account(account_id.clone()) {
      Some(it) => it,
      _ => panic!("Account not found"),
    };
    acc.get_balance_as_string()
  }

  pub fn assert_account_owner(&self, account_id: AccountId) {
    assert_eq!(
      env::predecessor_account_id(),
      account_id,
      "{}",
      ERR9_NOT_ALLOWED
    );
    
  }

  pub fn unlock_account(&mut self, account_id: AccountId, amount: u128) {
    let acc = match self.get_account(account_id.clone()) {
      Some(it) => it,
      _ => panic!("Account not found"),
    };
    acc.unlock(amount);
  }

  pub fn acc_transfers_count(&self, account_id: AccountId) -> i32 {
    let trs = self.get_account_transfers(account_id);
    let len = trs.len();
    return len.try_into().unwrap();
  }

  pub fn acc_trades_count(&self, account_id: AccountId) -> i32 {
    let trs = self.get_account_transfers(account_id);
    let len = trs.len();
    return len.try_into().unwrap();
  }

  pub fn acc_offers_count(&self, account_id: AccountId) -> i32 {
    let trs = self.get_account_transfers(account_id);
    let len = trs.len();
    return len.try_into().unwrap();
  }

  pub fn acc_pub_info(&self, account_id: AccountId) -> Option<PubAccountInfo>  {
    let acc = self.pub_get_account(account_id.clone());
    if acc.as_ref().is_some() {
      let acc_info = PubAccountInfo {
        id: acc.unwrap().id.clone(),
        likes: acc.unwrap().likes,
        trades: self.get_trades_length_by_account(account_id.clone()),
        transfers: self.get_transfers_len_by_account(account_id.clone()),
        offers: self.get_offers_len_by_account(account_id.clone())
          + self.get_t_offers_len_by_account(account_id.clone()),
        dislikes: acc.unwrap().dislikes,
        blocked_by: acc.unwrap().blocked_by,
        created_on: acc.unwrap().created_on,
      };
      return Some(acc_info);
    }
    return None;
  }
}
