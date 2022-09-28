#[cfg(test)]

pub mod tests {
  // use super::*;
  use crate::constants::*;
  use crate::*;
  use near_sdk::json_types::U128;
  use near_sdk::test_utils::test_env::alice;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::{testing_env, VMContext, ONE_NEAR};

  pub fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
      .current_account_id(master())
      .signer_account_id(dalmasonto())
      .predecessor_account_id(supercode())
      .is_view(is_view)
      .attached_deposit(0)
      .account_balance(0)
      .build()
  }

  #[test]
  fn test_test() {
    assert!(true);
  }

  #[test]
  fn register_new_account() {
    let _context = get_context(false);
    testing_env!(_context);

    let mut contract = Contract::new();

    let account_id = alice();
    assert_eq!(contract.accounts.len(), 0);
    let result = contract.register_new_account(account_id);

    assert_eq!(
      result,
      "Account registered successfully".to_string(),
      "ERROR: ACCOUNT REGISTRATION FAILED"
    );
  }

  #[test]
  fn get_account() {
    let _context = get_context(false);
    testing_env!(_context);

    let mut contract = Contract::new();

    contract.register_new_account(alice());

    let acc = contract.get_account(alice()).unwrap();

    assert_eq!(
      acc.id,
      alice(),
      "ERROR: ACCOUNT ID MISMATCH, USER NOT FOUND"
    );
  }

  #[test]
  fn deposit_into_account_and_contract() {
    let mut _context = get_context(false);
    _context.attached_deposit = ONE_NEAR * 5;
    testing_env!(_context);

    let mut contract = Contract::new();
    contract.register_new_account(supercode());

    contract.contract_deposit(&supercode());

    assert_eq!(
      contract.get_transfers_len(),
      1,
      "ERROR: TRANSFER COUNT SHOULD BE 1"
    );

    let acc = contract.get_account(supercode()).unwrap();

    assert_eq!(
      env::account_balance(),
      ONE_NEAR * 5,
      "ERROR: master() ACCOUNT BALANCE MISMATCH\n"
    );

    assert_eq!(
      acc.balance,
      ONE_NEAR * 5,
      "ERROR: supercode() ACCOUNT BALANCE MISMATCH\n"
    );
  }

  #[test]
  fn withdraw_from_contract() {
    let mut _context = get_context(false);
    _context.attached_deposit = 5 * ONE_NEAR;
    testing_env!(_context);

    let mut contract = Contract::new();

    contract.register_new_account(supercode());
    contract.contract_deposit(&supercode());
    contract.withdraw_near(U128::from(2 * ONE_NEAR));

    assert_eq!(
      contract.get_transfers_len(),
      2,
      "ERROR: TRANSFER COUNT SHOULD BE 2, (Indicating a deposit and withdraw)"
    );

    let acc = contract.get_account(supercode()).unwrap();

    assert_eq!(
      acc.balance,
      ONE_NEAR * 3,
      "ERROR: supercode() ACCOUNT BALANCE MISMATCH\n"
    );
    assert_eq!(
      env::account_balance(),
      ONE_NEAR * 3,
      "ERROR: master() ACCOUNT BALANCE MISMATCH\n"
    )
  }

  #[test]
  fn create_offer() {
    let mut _context = get_context(false);
    _context.attached_deposit = ONE_NEAR * 5;
    testing_env!(_context);

    let mut contract = Contract::new();
    contract.register_new_account(supercode());

    contract.contract_deposit(&supercode());

    contract.add_offer(
      "somestrangeid".to_string(),
      "buy".to_string(),
      supercode(),
      U128(10 * ONE_NEAR),
      U128(40 * ONE_NEAR),
      1.0,
      "M-Pesa".to_string(),
      "KES".to_string(),
      "Send money over".to_string()

    );
    contract.add_offer(
      "somestrangeid2".to_string(),
      "sell".to_string(),
      supercode(),
      U128(1 * ONE_NEAR),
      U128(5 * ONE_NEAR),
      1.0,
      "M-Pesa".to_string(),
      "KES".to_string(),
      "Send money over".to_string()
    );
    
    print!("{:#?}", contract);

    assert_eq!(contract.offers.len(), 2, "ERROR: OFFER COUNT SHOULD BE 2")

    // let acc = contract.get_account(supercode()).unwrap();

    // assert_eq!(
    //   acc.,
    //   ONE_NEAR * 5,
    //   "ERROR: supercode() ACCOUNT BALANCE MISMATCH\n"
    // );
  }
}
