use alloy_primitives::{ Address, Bytes, B256, U256 };
use revm::db::AccountState;
use revm::primitives::{ Account, AccountInfo, Bytecode, HashMap };
use revm::{ Database, DatabaseCommit };

use crate::error::{ BackendError, GoError };
use crate::memory::{ U8SliceView, UnmanagedVector };

use super::getter::Getter;
use super::setter::{ Getter, Setter };
use super::vtable::Db;

pub struct StateDB<'r> {
    pub db: &'r Db,
}

impl<'r> StateDB<'r> {
    pub fn new(db: &'r Db) -> Self {
        StateDB { db }
    }
}

impl<'db> Database for StateDB<'db> {
    type Error = BackendError;

    #[doc = " Get basic account information."]
    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let balance: U256 = self.get_balance(address).unwrap();
        let nonce: U256 = self.get_nonce(address).unwrap();
        let code_hash: U256 = self.get_code_hash(address).unwrap();
        let code = self.get_code(address).unwrap();
        Ok(
            Some(AccountInfo {
                balance,
                nonce,
                code_hash,
                code: Bytecode::new_raw_checked(Bytes::copy_from_slice(&code)),
            })
        )
    }

    #[doc = " Get account code by its hash."]
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        Err(BackendError::UnreachableCall)
    }

    #[doc = " Get storage value of address at index."]
    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        self.get_state(address, index)
    }

    #[doc = " Get block hash by block number."]
    fn block_hash(&mut self, number: u64) -> Result<B256, Self::Error> {
        // TODO: UNSUPPORTED?
    }
}

impl<'a> DatabaseCommit for StateDB<'a> {
    #[doc = " Commit changes to the database."]
    fn commit(&mut self, changes: HashMap<Address, Account>) {
        for (address, mut account) in changes {
            if !account.is_touched() {
                continue;
            }
            if account.is_selfdestructed() {
                self.self_destruct(address);
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
