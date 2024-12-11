use std::hash::Hash;

use alloy_primitives::{ Address, Bytes, B256, U256 };
use revm::db::AccountState;
use revm::precompile::bn128::add;
use revm::primitives::{ Account, AccountInfo, BlockEnv, Bytecode, HashMap };
use revm::{ Database, DatabaseCommit };

use crate::error::{ BackendError, GoError };
use crate::memory::{ U8SliceView, UnmanagedVector };

use super::statedb::StateDB;
use super::vtable::Db;

impl<'db> Database for StateDB<'db> {
    type Error = BackendError;

    #[doc = " Get basic account information."]
    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let mut output = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_account)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();
        let default = || format!("Failed to get account info from the db");
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    #[doc = " Get account code by its hash."]
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let mut error_msg = UnmanagedVector::default();
        let mut output = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_code_by_hash)(
                self.db.state,
                U8SliceView::new(Some(code_hash)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();
        let default = || format!("Failed to get code from the db");
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    #[doc = " Get storage value of address at index."]
    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let mut error_msg = UnmanagedVector::default();
        let mut output = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_storage)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(index)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();
        let default = || format!("Failed to get storage from the db");
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    #[doc = " Get block hash by block number."]
    fn block_hash(&mut self, number: u64) -> Result<B256, Self::Error> {
        let mut error_msg = UnmanagedVector::default();
        let mut output = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_block_hash)(
                self.db.state,
                number,
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();
        let default = || format!("Failed to get block hash from the db");
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }
}

impl<'a> DatabaseCommit for StateDB<'a> {
    #[doc = " Commit changes to the database."]
    fn commit(&mut self, changes: HashMap<Address, Account>) {
        let changed_codes = vec![];
        let changed_storages = vec![];
        let changed_accounts = vec![];
        let deleted_state = vec![];
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
        // commit by ffi call
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.commit)(
            self.db.state,
            U8SliceView::new(changed_codes),
            U8SliceView::new(changed_storages),
            U8SliceView::new(changed_accounts),
            U8SliceView::new(deleted_state),
            &mut error_msg as *mut UnmanagedVector
        );
    }
}
