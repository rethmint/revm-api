// summarize new account to sync with account modules
use std::collections::HashMap;

use alloy_primitives::{ Address, B256 };
use flatbuffer_types::{ bank::finish_bank_records_buffer, AccountRecord, AccountRecordArgs };
use revm::primitives::{ Account, EvmState, KECCAK_EMPTY };

// summarize new account to sync with account modules
pub fn account_end_handler(state: EvmState) -> Vec<AccountRecord> {
    let mut records = Vec::new();
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(10);
    state.iter().for_each(|(addres, account)| {
        // track only EOA without CA
        if account.is_created() && account.info.code_hash.eq(&KECCAK_EMPTY) {
            let args = AccountRecord::create(
                &mut builder,
                &(AccountRecordArgs {
                    address,
                    sequence: account.info.nonce,
                })
            );
            finish_bank_records_buffer(&mut builder, args);
        }
    });
    records
}

// add test
