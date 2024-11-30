use alloy_primitives::{Address, Bytes, B256, U256};
use revm::{
    primitives::{Account, AccountInfo, Bytecode, HashMap},
    Database, DatabaseCommit,
};

use crate::error::BackendError;

use super::{compress_account_info, parse_account_info, EvmStoreKey, GoStorage, Storage};

impl<'db> Database for GoStorage<'db> {
    type Error = BackendError;

    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let account_key = EvmStoreKey::Account(address).key();
        let default = vec![0u8; 72];
        let output = self.get(account_key.as_slice(), &default)?;
        if output == default {
            Ok(None)
        } else {
            Ok(Some(parse_account_info(output)))
        }
    }

    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let storage_key = EvmStoreKey::Storage(address, index).key();
        let storage_key_slice = storage_key.as_slice();
        let default = vec![0u8; 32];
        let output = self.get(storage_key_slice, &default)?;

        Ok(U256::from_be_slice(&output))
    }

    fn block_hash(&mut self, number: u64) -> Result<B256, Self::Error> {
        let block_key = EvmStoreKey::Block(number).key();
        let block_key_slice = block_key.as_slice();
        let default = vec![0u8; 32];
        let output = self.get(block_key_slice, &default)?;

        Ok(B256::from_slice(&output))
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let code_key = EvmStoreKey::Code(code_hash).key();
        let code_key_slice = code_key.as_slice();
        let default = Vec::new();
        let output = self.get(code_key_slice, &default)?;

        Ok(Bytecode::new_raw(Bytes::from(output)))
    }
}

// COMM: cold , selfdestructed , LoadedAsNotExisting are not supported
impl<'a> DatabaseCommit for GoStorage<'a> {
    fn commit(&mut self, changes: HashMap<Address, Account>) {
        for (address, account) in changes.iter() {
            if !account.is_touched() {
                // filter Loaded
                continue;
            }
            let is_newly_created = account.is_created();
            // account info update
            let account_key = EvmStoreKey::Account(*address).key();
            let account_key_slice = account_key.as_slice();

            let account_info_vec: Vec<u8> = compress_account_info(account.info.clone());
            self.set(account_key_slice, &account_info_vec).unwrap();

            if is_newly_created && !account.info.is_empty_code_hash() {
                let code_hash_key = EvmStoreKey::Code(account.info.code_hash()).key();
                let code_hash_key_slice = code_hash_key.as_slice();
                let _ = self.set(
                    code_hash_key_slice,
                    account.info.clone().code.unwrap().bytes_slice(),
                );
            }

            // storage cache commit on value changed
            let storage = account.storage.clone();
            for (index, slot) in storage {
                if slot.present_value == slot.original_value {
                    continue;
                }
                let storage_key = EvmStoreKey::Storage(*address, index).key();
                let storage_key_slice = storage_key.as_slice();

                let mut vec = Vec::with_capacity(72);
                let slot_present_value_vec = slot.present_value.to_be_bytes_vec();
                vec.extend(&slot_present_value_vec);

                self.set(storage_key_slice, &vec).unwrap();
            }
        }
    }
}
