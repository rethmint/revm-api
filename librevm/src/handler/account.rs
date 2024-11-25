// summarize new account to sync with account modules
use std::collections::HashMap;

use alloy_primitives::Address;
use revm::primitives::{ Account, EvmState };

// summarize new account to sync with account modules
pub fn record_new_account(state: EvmState) -> HashMap<Address, Account> {
    state
        .iter()
        .filter(|(addr, account)| account.is_created())
        .collect()
}
