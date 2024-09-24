use anyhow::Ok;
use revm::db::AccountStatus;
use revm::{Database, DatabaseCommit};
use revm_primitives::{AccountInfo, AccountStatus, Address, Bytecode, B256};
use types::{AccessPath, BackendError};

use crate::db::Db;
use crate::error::GoError;
use crate::memory::{U8SliceView, UnmanagedVector};
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

impl<'r> Storage for GoStorage<'r> {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.read_db)(
            self.db.state,
            U8SliceView::new(Some(key)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        // We destruct the UnmanagedVector here, no matter if we need the data.
        let output = output.consume();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || {
            format!(
                "Failed to read a key in the db: {}",
                String::from_utf8_lossy(key)
            )
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }

        Ok(output)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.write_db)(
            self.db.state,
            U8SliceView::new(Some(key)),
            U8SliceView::new(Some(value)),
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        // return complete error message (reading from buffer for GoError::Other)
        let default = || {
            format!(
                "Failed to set a key in the db: {}",
                String::from_utf8_lossy(key)
            )
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn remove(&mut self, key: &[u8]) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.remove_db)(
            self.db.state,
            U8SliceView::new(Some(key)),
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        let default = || {
            format!(
                "Failed to delete a key in the db: {}",
                String::from_utf8_lossy(key)
            )
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }
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
// ACCOUNT_PREFIX(B1) + {address(B20)} => ACCOUNT INFO {balance(B32)(0) | nonce(B8)(1) | code_hash(B32)(2)}
// CODE_PREFIX(B1) + {code_hash(B32)} => vm bytecode
// STORAGE_PREFIX(B1) + {address(B20)} + {index(B32)} => [32]byte(value)
// BLOCK_PREFIX(B1) + block_num(B8) => block_hash

trait EvmStoreKey {
    const ACCOUNT_PREFIX: u8 = 1;
    const CODE_PREFIX: u8 = 2;
    const STORAGE_PREFIX: u8 = 3;
    const BLOCK_PREFIX: u8 = 4;

    fn account_key(address: Address) -> &[u8];
    fn code_key(code_hash: B256) -> &[u8];
    fn storage_key(address: Address, index: revm_primitives::U256) -> &[u8];
    fn block_hash_key(block_num: u64) -> &[u8];
}

impl EvmStoreKey for Address {
    fn account_key(address: revm_primitives::Address) -> &[u8] {
        let mut result = vec![EvmStoreKey::ACCOUNT_PREFIX];
        result.append(&mut address.to_vec());
        &result
    }

    fn code_key(code_hash: B256) -> &[u8] {
        let mut result = vec![EvmStoreKey::CODE_PREFIX];
        result.append(&mut code_hash.to_vec());
        &result
    }

    fn storage_key(address: revm_primitives::Address, index: revm_primitives::U256) -> &[u8] {
        let mut result = vec![EvmStoreKey::STORAGE_PREFIX];
        result.append(&mut address.to_vec());
        &result
    }

    fn block_hash_key(block_num: u64) -> &[u8] {
        let mut result = vec![EvmStoreKey::BLOCK_PREFIX];
        result.append(&mut block_num.to_be_bytes().to_vec());
        &result
    }
}

impl ByteKey for AccountInfo {
    fn extract(
        output: Result<Option<Vec<u8>>, BackendError>,
    ) -> Result<Option<Self>, BackendError> {
        match output {
            Ok(Some(vec)) => {
                let balance =
                    U256::from_big_endian(output.get(0..32).ok_or("fail to extract balance")?);
                let nonce = u64::from_be_bytes(output.get(32..40).ok_or("fail to extract nonce")?);
                let code_hash: B256 = output.get(40..72).ok_or("fail to extract code_hash")?;
                let code = Bytecode::default();
                AccountInfo::new(balance, nonce, code_hash, code).without_code()
            }
            Ok(None) => Err(BackendError::new("fail to extract")),
        }
    }

    fn compress(&self) -> &[u8] {
        let mut result = Vec::with_capacity(72);

        // balance: U256 (32 bytes)
        let mut balance_bytes = [0u8; 32];
        self.balance.to_big_endian(&mut balance_bytes);
        result.extend_from_slice(&balance_bytes);

        // nonce: u64 (8 bytes)
        let nonce_bytes = self.nonce.to_be_bytes();
        result.extend_from_slice(&nonce_bytes);

        // code_hash: B256 (32 bytes)
        result.extend_from_slice(&self.code_hash);

        &result
    }
}

// TODO: get with default value
impl<'DB> Database for GoStorage<'DB> {
    type Error = BackendError;

    fn basic(
        &mut self,
        address: revm_primitives::Address,
    ) -> Result<Option<revm_primitives::AccountInfo>, Self::Error> {
        let output = self.get(EvmStoreKey::account_key(address));
        Ok(extract(output))
    }

    fn storage(
        &mut self,
        address: revm_primitives::Address,
        index: revm_primitives::U256,
    ) -> Result<revm_primitives::U256, Self::Error> {
        let output = self.get(EvmStoreKey::storage_key(address, index));
        Ok(output)
    }

    fn block_hash(&mut self, number: u64) -> Result<revm_primitives::B256, Self::Error> {
        let output = self.get(EvmStoreKey::block_hash_key(number));
        Ok(output)
    }

    fn code_by_hash(
        &mut self,
        code_hash: revm_primitives::B256,
    ) -> Result<revm_primitives::Bytecode, Self::Error> {
        let output = self.get(EvmStoreKey::code_key(code_hash));
        Ok(output);
    }
}
// COMM: Cold, SelfDestructed and LoadedAsNotExisting are not supported
// handle Loaded(no storage change) / Created / Touched
impl<'a> DatabaseCommit for GoStorage<'a> {
    fn commit(&mut self, changes: std::collections::HashMap<Address, revm_primitives::Account>) {
        for (address, account) in changes.iter() {
            if !account.is_touched() {
                // filter Loaded
                continue;
            }
            let is_newly_created = account.is_created();
            // account info update
            let account_key = EvmStoreKey::account_key(address);
            self.set(account_key, account.info.compress());

            if !is_newly_created {
                // storage cache commit on value changed
                let storage = account.storage;
                for (index, slot) in storage {
                    if slot.present_value == slot.original_value {
                        continue;
                    }
                    let storage_key = EvmStoreKey::storage_key(address, index);
                    self.set(storage_key, slot.present_value);
                }
            }
        }
    }
}
