use anyhow::{format_err, Error, Result};
use ethers_core::types::Address;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum AccountType {
    BaseAccount = 0,
    ObjectAccount = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    address: Address,
    account_number: u64,
    account_type: u8,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Accounts(Vec<(Address, u64, u8)>);

impl Accounts {
    pub fn new(events: Vec<(Address, u64, u8)>) -> Accounts {
        Self(events)
    }

    pub fn into_inner(self) -> Vec<Account> {
        self.0
            .into_iter()
            .map(|v| Account {
                address: v.0,
                account_number: v.1,
                account_type: v.2,
            })
            .collect()
    }
}

impl AsRef<Vec<(Address, u64, u8)>> for Accounts {
    fn as_ref(&self) -> &Vec<(Address, u64, u8)> {
        &self.0
    }
}

impl AccountType {
    pub fn is_valid(value: u8) -> bool {
        Self::try_from(value).is_ok()
    }
}

impl TryFrom<u8> for AccountType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccountType::BaseAccount),
            1 => Ok(AccountType::ObjectAccount),
            _ => Err(format_err!("Invalid AccountType")),
        }
    }
}
