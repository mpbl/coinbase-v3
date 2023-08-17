//! Structures & Enums to store Coinbase's Accounts

use bigdecimal::BigDecimal;
use serde_derive::Deserialize;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use uuid::Uuid;

use crate::DateTime;

/// Possible types for Coinbase's accounts.
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    AccountTypeUnspecified,
    AccountTypeCrypto,
    AccountTypeFiat,
    AccountTypeVault,
}

/// Structure to deserialize Coinbase's accounts.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Account {
    pub uuid: Uuid,
    pub name: String,
    pub currency: String,
    pub available_balance: Balance,
    pub default: bool,
    pub active: bool,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub r#type: AccountType,
    pub ready: bool,
    pub hold: Balance,
}

/// Structure to deserialize balances stored in a Coinbase's account.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Balance {
    /// Not store as an `f64` as number of decimals might be currency dependant and arbitrary
    pub value: BigDecimal,
    pub currency: String,
}

/// Structure to deserialize CB's response to a request for multiple accounts.
///
/// Calls to this [Client][`crate::client::CbClient`]'s API will not return this type. It will unpack the
/// inner `accounts` and return it.
///
/// `has_next` and `cursor` are used for pagination.
#[derive(Deserialize, Debug)]
pub struct AccountsResponse {
    pub accounts: Vec<Account>,
    pub has_next: bool,
    pub cursor: String,
    pub size: i32, // i32 as per api reference
}

/// Structure to deserialize CB's response to a request for a single account.
///
/// Calls to this [Client][`crate::client::CbClient`]'s API will not return this type. It will unpack the
/// inner `accounts` and return it.
#[derive(Deserialize, Debug)]
pub struct AccountResponse {
    pub account: Account,
}

//=========== TESTS ===========================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::ToPrimitive;

    #[test]
    fn test_account_deserialize() {
        let input = r##"[
      {
        "uuid": "9dd482e4-d8ce-46f7-a261-281843bd2855",
        "name": "SOL Wallet",
        "currency": "SOL",
        "available_balance": { "value": "70.313593992", "currency": "SOL" },
        "default": true,
        "active": true,
        "created_at": "2023-06-07T17:30:40.425Z",
        "deleted_at": null,
        "type": "ACCOUNT_TYPE_CRYPTO",
        "ready": true,
        "hold": { "value": "0", "currency": "SOL" }
      }
]"##;

        let accounts: Vec<Account> = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(
            accounts[0].uuid.to_string(),
            "9dd482e4-d8ce-46f7-a261-281843bd2855"
        );
    }

    #[test]
    fn test_balance_deserialize() {
        let input = r##"{ "value": "70.313593992", "currency": "SOL" }"##;
        let balance: Balance = serde_json::from_slice(input.as_bytes()).unwrap();
        assert!((balance.value.to_f64().unwrap() - 70.313593992f64).abs() < 0.000000001);
    }

    #[test]
    fn test_account_type_deserialize() {
        let input = r##""ACCOUNT_TYPE_UNSPECIFIED""##;
        let result: AccountType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, AccountType::AccountTypeUnspecified);

        let input = r##""ACCOUNT_TYPE_CRYPTO""##;
        let result: AccountType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, AccountType::AccountTypeCrypto);

        let input = r##""ACCOUNT_TYPE_FIAT""##;
        let result: AccountType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, AccountType::AccountTypeFiat);

        let input = r##""ACCOUNT_TYPE_VAULT""##;
        let result: AccountType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, AccountType::AccountTypeVault);
    }

    #[test]
    fn test_account_type_serialize() {
        let expected = r##""ACCOUNT_TYPE_CRYPTO""##;
        assert_eq!(
            expected,
            serde_json::to_string(&AccountType::AccountTypeCrypto).unwrap()
        );
    }
}
