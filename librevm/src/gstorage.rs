use bytes::Bytes;
use revm::Database;
use storage::StateView;
use types::{AccessPath, BackendError};

use crate::db::Db;
use crate::error::GoError;
use crate::memory::{U8SliceView, UnmanagedVector};

use anyhow::anyhow;

/// Access to the VM's backend storage, i.e. the chain
pub trait Storage {
    #[allow(dead_code)]
    /// Returns Err on error.
    /// Returns Ok(None) when key does not exist.
    /// Returns Ok(Some(Vec<u8>)) when key exists.
    ///
    /// Note: Support for differentiating between a non-existent key and a key with empty value
    /// is not great yet and might not be possible in all backends. But we're trying to get there.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, BackendError>;

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), BackendError>;

    /// Removes a database entry at `key`.
    ///
    /// The current interface does not allow to differentiate between a key that existed
    /// before and one that didn't exist. See https://github.com/CosmWasm/cosmwasm/issues/290
    fn remove(&mut self, key: &[u8]) -> Result<(), BackendError>;
}

pub struct GoStorage<'r> {
    db: &'r Db,
}

impl<'r> GoStorage<'r> {
    pub fn new(db: &'r Db) -> Self {
        GoStorage { db }
    }
}

impl<'DB> Database for GoStorage<'DB> {
    type Error = BackendError;

    fn basic(
        &mut self,
        address: revm_primitives::Address,
    ) -> Result<Option<revm_primitives::AccountInfo>, Self::Error> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let mut used_gas = 0_u64;

        let read_db = self
            .db
            .vtable
            .read_db
            .expect("vtable function 'read_db' not set");

        let go_error: GoError = read_db(
            self.db.state,
            U8SliceView::new(Some(address)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();

        let output = output.consume();
        let default = || {
            format!(
                "Failed to read an address in the db: {}",
                String::from_utf8_lossy(address)
            )
        };

        Ok(output.into())
    }

    fn storage(
        &mut self,
        address: revm_primitives::Address,
        index: revm_primitives::U256,
    ) -> Result<revm_primitives::U256, Self::Error> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let mut used_gas = 0_u64;

        let read_db = self
            .db
            .vtable
            .read_db
            .expect("vtable function 'read_db' not set");

        let go_error: GoError = read_db(
            self.db.state,
            U8SliceView::new(Some(address)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();

        let output = output.consume();
        let default = || {
            format!(
                "Failed to read an address in the db: {}",
                String::from_utf8_lossy(address)
            )
        };

        Ok(output.map(|v| v.get(index).unwrap()).into())
    }

    fn block_hash(&mut self, number: u64) -> Result<revm_primitives::B256, Self::Error> {
        // TODO: implement this after verifying that kvvalue can be imported as go storage
        todo!();
    }

    fn code_by_hash(
        &mut self,
        code_hash: revm_primitives::B256,
    ) -> Result<revm_primitives::Bytecode, Self::Error> {
        // TODO: implement this after verifying that kvvalue can be imported as go storage
        todo!();
    }
}
