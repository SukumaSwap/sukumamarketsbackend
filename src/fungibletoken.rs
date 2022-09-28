use core::panic;

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{ext_contract, near_bindgen, Promise, PromiseOrValue, PromiseResult, ONE_YOCTO};

use crate::constants::{GAS_FOR_BASIC_OP, GAS_FOR_FT_TRANSFER};
use crate::*;

// #[near_bindgen]
#[ext_contract(ext_self)]
trait ContractCallBacks {
  fn deposit_tokens(
    &mut self,
    account_id: AccountId,
    token_id: AccountId,
    amount: U128,
  ) -> PromiseOrValue<U128>;

  fn withdraw_tokens(
    &mut self,
    account_id: AccountId,
    token_id: AccountId,
    amount: U128,
    chat_id: String,
  ) -> PromiseOrValue<U128>;
  fn withdraw_asset(
    &mut self,
    acc: AccountId,
    token: AccountId,
    amount: U128
  ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl ContractCallBacks for Contract {
  fn deposit_tokens(
    &mut self,
    account_id: AccountId,
    token_id: AccountId,
    amount: U128,
  ) -> PromiseOrValue<U128> {
    let account = self.get_account(account_id.clone());
    if account.as_ref().is_some() {
      account
        .unwrap()
        .deposit_tokens(token_id.clone(), u128::from(amount.clone()));
    } else {
      self.register_new_account(account_id.clone());
      let account_ = self.get_account(account_id.clone()).unwrap();
      account_.deposit_tokens(token_id.clone(), u128::from(amount.clone()));
    }
    PromiseOrValue::Value(U128(0))
  }

  fn withdraw_tokens(
    &mut self,
    account_id: AccountId,
    token_id: AccountId,
    amount: U128,
    chat_id: String,
  ) -> PromiseOrValue<U128> {
    assert!(
      env::predecessor_account_id() == env::current_account_id(),
      "{}",
      ERR9_NOT_ALLOWED
    );
    let account = self.get_account(account_id.clone());
    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Successful(_) => account.unwrap().token_release(token_id, u128::from(amount)),
      PromiseResult::Failed => {
        let mut chat = self.tokenchats.remove(&chat_id.clone()).unwrap();
        chat.released = false;
        chat.received = false;
        self.tokenchats.insert(&chat_id.clone(), &chat.clone());
      }
    }
    PromiseOrValue::Value(U128(0))
  }

  fn withdraw_asset(
    &mut self,
    acc: AccountId,
    token: AccountId,
    amount: U128
  ) -> PromiseOrValue<U128> {
    assert!(
      env::predecessor_account_id() == env::current_account_id(),
      "{}",
      ERR9_NOT_ALLOWED
    );
    let account = self.get_account(acc.clone());
    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Successful(_) => account.unwrap().token_release(token, u128::from(amount)),
      PromiseResult::Failed => {
        panic!("Transaction failed");
        // PromiseOrValue::Value(U128(0))
      }
    }
    PromiseOrValue::Value(U128(0))
  }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
  /// Callback on receiving tokens by this contract.
  // #[payable]
  #[allow(unreachable_code)]
  fn ft_on_transfer(
    &mut self,
    sender_id: AccountId,
    amount: U128,
    msg: String,
  ) -> PromiseOrValue<U128> {
    env::log_str(msg.as_str());
    let token_id = env::predecessor_account_id();
    near_sdk::PromiseOrValue::Promise(
      Self::ext(env::current_account_id()).deposit_tokens(sender_id, token_id, amount),
    )
  }
}

#[near_bindgen]
impl Contract {

  #[private]
  pub fn send_tokens(
    &mut self,
    from: AccountId,
    to: AccountId,
    token: AccountId,
    amount: U128,
    chat_id: String,
  ) -> Promise {
    let cross_contract_call = Promise::new(token.clone()).function_call(
      "ft_transfer".to_string(),
      json!({ "receiver_id": to, "amount":  amount.clone()})
        .to_string()
        .into_bytes(),
      ONE_YOCTO,
      GAS_FOR_FT_TRANSFER,
    );

    let callback = Promise::new(env::current_account_id()).function_call(
      "withdraw_tokens".to_string(),
      json!({ "account_id": from,"token_id": token.clone(), "amount":  amount, "chat_id": chat_id})
        .to_string()
        .into_bytes(),
      0,
      GAS_FOR_BASIC_OP,
    );

    cross_contract_call.then(callback)
  }

  #[private]
  pub fn withdraw_named_asset(
    &mut self,
    token: AccountId,
    amount: U128
  ) -> Promise {

    let me = env::predecessor_account_id();

    let cross_contract_call = Promise::new(token.clone()).function_call(
      "ft_transfer".to_string(),
      json!({ "receiver_id": me.clone(), "amount":  amount.clone()})
        .to_string()
        .into_bytes(),
      ONE_YOCTO,
      GAS_FOR_FT_TRANSFER,
    );

    let callback = Promise::new(env::current_account_id()).function_call(
      "withdraw_asset".to_string(),
      json!({ "acc": me,"token": token.clone(), "amount":  amount})
        .to_string()
        .into_bytes(),
      0,
      GAS_FOR_BASIC_OP,
    );

    cross_contract_call.then(callback)
  }
}
