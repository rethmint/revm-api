use alloy_primitives::{ Address, Bytes, B256, U256 };
use revm::{ primitives::{ Account, AccountInfo, Bytecode, HashMap }, Database, DatabaseCommit };

use crate::error::BackendError;

use super::{ compress_account_info, parse_account_info, EvmStoreKey, GoStorage, Storage };

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
        for (address, mut account) in changes {
            if !account.is_touched() {
                continue;
            }
            if account.is_selfdestructed() {
                let db_account = self.accounts.entry(address).or_default();
                db_account.storage.clear();
                db_account.account_state = AccountState::NotExisting;
                db_account.info = AccountInfo::default();
                continue;
            }
            let is_newly_created = account.is_created();
            self.insert_contract(&mut account.info);

            let db_account = self.accounts.entry(address).or_default();
            db_account.info = account.info;

            db_account.account_state = if is_newly_created {
                db_account.storage.clear();
                AccountState::StorageCleared
            } else if db_account.account_state.is_storage_cleared() {
                // Preserve old account state if it already exists
                AccountState::StorageCleared
            } else {
                AccountState::Touched
            };
            db_account.storage.extend(
                account.storage.into_iter().map(|(key, value)| (key, value.present_value()))
            );
        }
    }
}
