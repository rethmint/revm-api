use bytes::Bytes;
use k256::elliptic_curve::rand_core::block;
use k256::pkcs8::der::Encode;
use revm::Database;
use revm_precompile::bn128::add;
use revm_primitives::B256;
use storage::StateView;
use types::{ AccessPath, BackendError };

use crate::db::Db;
use crate::error::GoError;
use crate::memory::{ U8SliceView, UnmanagedVector };

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
// KVStore
// TODO: key padding to query
// ACCOUNT_PREFIX(B1) + {address(B20)} => ACCOUNT INFO {balance(B256)(0) | nonce(B256)(1) | code_hash(B256)(2)}
// CODE_PREFIX(B1) + {code_hash(B32)} => vm bytecode
// STORAGE_PREFIX(B1) + {address(B20)} + {index(B32)} => [32]byte(value)
// BLOCK_PREFIX(B1) + block_num(B8) => block_hash
const ACCOUNT_PREFIX: u8 = 1;
const CODE_PREFIX: u8 = 2;
const STORAGE_PREFIX: u8 = 3;
const BLOCK_PREFIX: u8 = 4;
impl EvmStoreKey {
    fn account_key(address: revm_primitives::Address) -> Vec<u8> {
        let mut result = vec![ACCOUNT_PREFIX];
        return result.append(&mut address.to_vec());
    }

    fn code_key(code_hash: B256) -> Vec<u8> {
        let mut result = vec![CODE_PREFIX];
        return result.append(&mut code_hash);
    }

    fn storage_key(address: revm_primitives::Address, index: revm_primitives::U256) -> Vec<u8> {
        let mut result = vec![ACCOUNT_PREFIX];
        return result.append(&mut address.to_vec());
    }

    fn block_hash_key(block_num: u64) -> Vec<u8> {
        let mut result = vec![block_num];
        return result.append(block_num);
    }
}
impl<'DB> Database for GoStorage<'DB> {
    type Error = BackendError;

    fn basic(
        &mut self,
        address: revm_primitives::Address
    ) -> Result<Option<revm_primitives::AccountInfo>, Self::Error> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();

        let read_db = self.db.vtable.read_db.expect("vtable function 'read_db' not set");

        let go_error: GoError = read_db(
            self.db.state,
            U8SliceView::new(EvmStoreKey::account_key(address)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector
        ).into();

        let output = output.consume();
        let default = || {
            format!("Failed to read an address in the db: {}", String::from_utf8_lossy(address))
        };

        Ok(output.into())
    }

    fn storage(
        &mut self,
        address: revm_primitives::Address,
        index: revm_primitives::U256
    ) -> Result<revm_primitives::U256, Self::Error> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();

        let read_db = self.db.vtable.read_db.expect("vtable function 'read_db' not set");

        let go_error: GoError = read_db(
            self.db.state,
            U8SliceView::new(EvmStoreKey::storage_key(address, index)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector
        ).into();

        let output = output.consume();

        Ok(output.into())
    }

    fn block_hash(&mut self, number: u64) -> Result<revm_primitives::B256, Self::Error> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();

        let read_db = self.db.vtable.read_db.expect("vtable function 'read_db' not set");

        let go_error: GoError = read_db(
            self.db.state,
            U8SliceView::new(EvmStoreKey::block_hash_key(number)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector
        ).into();

        let output = output.consume();

        Ok(output.into())
    }

    fn code_by_hash(
        &mut self,
        code_hash: revm_primitives::B256
    ) -> Result<revm_primitives::Bytecode, Self::Error> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let mut used_gas = 0_u64;

        let read_db = self.db.vtable.read_db.expect("vtable function 'read_db' not set");

        let go_error: GoError = read_db(
            self.db.state,
            U8SliceView::new(EvmStoreKey::code_key(code_hash)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector
        ).into();

        let output = output.consume();

        Ok(output.into())
    }
}
