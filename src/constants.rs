use near_sdk::{
  AccountId, Gas, StorageUsage,
};

const U128_STORAGE: StorageUsage = 16;
const U64_STORAGE: StorageUsage = 8;
const U32_STORAGE: StorageUsage = 4;

const ACC_ID_STORAGE: StorageUsage = 64;

const ACC_ID_AS_KEY_STORAGE: StorageUsage = ACC_ID_STORAGE + 4;
// const KEY_PREFIX_ACC: StorageUsage = 64;

const ACC_ID_AS_CLT_KEY_STORAGE: StorageUsage = ACC_ID_AS_KEY_STORAGE + 1;

pub const INIT_ACCOUNT_STORAGE: StorageUsage =
  ACC_ID_AS_CLT_KEY_STORAGE + 1 + U128_STORAGE + U32_STORAGE + U32_STORAGE + U64_STORAGE;

pub const GAS_FOR_BASIC_OP: Gas = Gas(10_000_000_000_000);

pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(20_000_000_000_000);

pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + 20_000_000_000_000);

pub const GAS_FOR_FT_TRANSFER: Gas = Gas(20_000_000_000_000);

// All the accounts that we may need during testing
pub fn supercode() -> AccountId {
  "supercode.testnet".to_string().try_into().unwrap()
}

pub fn dalmasonto() -> AccountId {
  "dalmasonto.testnet".to_string().try_into().unwrap()
}

pub fn usdn() -> AccountId {
  "usdn.testnet".to_string().try_into().unwrap()
}

pub fn master() -> AccountId {
  "master.testnet".to_string().try_into().unwrap()
}

pub fn master1() -> AccountId {
  "master1.testnet".to_string().try_into().unwrap()
}

#[allow(non_snake_case)]
pub fn getAccountId(account_id: String) -> AccountId {
  account_id.try_into().unwrap()
}
