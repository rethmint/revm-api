use std::collections::HashMap;
use alloy_primitives::{ Address, Bytes, B256, U256 };
use revm::bytecode::LegacyRawBytecode;
use revm::{ Database, DatabaseCommit };
use state::{ Account, AccountInfo, Bytecode };
use crate::db::Db;
use crate::error::{ BackendError, GoError };
use crate::memory::{ U8SliceView, UnmanagedVector };
/// Access to the VM's backend storage, i.e. the chain
pub trait Storage {
    #[allow(dead_code)]
    /// Returns Err on error.
    /// Returns Ok(None) when key does not exist.
    /// Returns Ok(Some(Vec<u8>)) when key exists.
    ///
    /// Note: Support for differentiating between a non-existent key and a key with empty value
    /// is not great yet and might not be possible in all backends. But we're trying to get there.
    fn get(&self, key: &[u8], default: &[u8]) -> Result<Vec<u8>, BackendError>;

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), BackendError>;

    /// Removes a database entry at `key`.
    ///
    /// The current interface does not allow to differentiate between a key that existed
    /// before and one that didn't exist. See https://github.com/CosmWasm/cosmwasm/issues/290
    #[allow(dead_code)]
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
// ACCOUNT_PREFIX(B1) + {address(B20)} => ACCOUNT INFO {balance(B64)(0) | nonce(B256)(1) | code_hash(B256)(2)}
// CODE_PREFIX(B1) + {code_hash(B32)} => vm bytecode
// STORAGE_PREFIX(B1) + {address(B20)} + {index(B32)} => [32]byte(value)
// BLOCK_PREFIX(B1) + block_num(B8) => block_hash

enum EvmStoreKeyPrefix {
    Account,
    Code,
    Storage,
    Block,
}

impl From<EvmStoreKeyPrefix> for u8 {
    fn from(value: EvmStoreKeyPrefix) -> Self {
        match value {
            EvmStoreKeyPrefix::Account => 1,
            EvmStoreKeyPrefix::Code => 2,
            EvmStoreKeyPrefix::Storage => 3,
            EvmStoreKeyPrefix::Block => 4,
        }
    }
}

type CodeHash = B256;
type StorageIndex = U256;
type BlockNum = u64;

enum EvmStoreKey {
    Account(Address),
    Code(CodeHash),
    Storage(Address, StorageIndex),
    Block(BlockNum),
}

impl EvmStoreKey {
    fn key(self) -> Vec<u8> {
        match self {
            Self::Account(addr) => {
                let mut result: Vec<u8> = vec![EvmStoreKeyPrefix::Account.into()];

                result.append(&mut addr.to_vec());
                result
            }
            Self::Code(addr) => {
                let mut result = vec![EvmStoreKeyPrefix::Code.into()];
                result.append(&mut addr.to_vec());
                result
            }
            Self::Storage(addr, idx) => {
                let mut result = vec![EvmStoreKeyPrefix::Storage.into()];
                result.append(&mut addr.to_vec());
                result.append(&mut idx.to_be_bytes::<32>().to_vec());
                result
            }
            Self::Block(block_num) => {
                let mut result = vec![EvmStoreKeyPrefix::Block.into()];
                result.append(&mut block_num.to_be_bytes().to_vec());
                result
            }
        }
    }
}

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

        Ok(Bytecode::LegacyRaw(LegacyRawBytecode::from(Bytes::from(output))))
    }
}

impl<'r> Storage for GoStorage<'r> {
    fn get(&self, key: &[u8], default: &[u8]) -> Result<Vec<u8>, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .read_db)(
                self.db.state,
                U8SliceView::new(Some(key)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .write_db)(
                self.db.state,
                U8SliceView::new(Some(key)),
                U8SliceView::new(Some(value)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();
        // return complete error message (reading from buffer for GoError::Other)
        let default = || {
            format!("Failed to set a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn remove(&mut self, key: &[u8]) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .remove_db)(
                self.db.state,
                U8SliceView::new(Some(key)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();
        let default = || {
            format!("Failed to delete a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }
}

// compress account info data with bigedien order, not included bytecode
fn compress_account_info(info: AccountInfo) -> Vec<u8> {
    let mut vec = Vec::with_capacity(72);

    let balance_be_bytes = info.balance.to_be_bytes_vec();
    vec.extend(&balance_be_bytes);

    let nonce_be_bytes = info.nonce.to_be_bytes();
    vec.extend_from_slice(&nonce_be_bytes);

    vec.extend(info.code_hash.to_vec());

    vec
}
// return Account info with no code
fn parse_account_info(value: Vec<u8>) -> AccountInfo {
    let balance_bytes: [u8; 32] = value[0..32].try_into().unwrap();
    let balance = U256::from_be_slice(&balance_bytes);

    let nonce_bytes: [u8; 8] = value[32..40].try_into().unwrap();
    let nonce = u64::from_be_bytes(nonce_bytes);

    let code_hash_bytes: [u8; 32] = value[40..72]
        .try_into()
        .expect("Code hash is not long enough size of code hash");
    let code_hash = B256::from(code_hash_bytes);

    AccountInfo::new(balance, nonce, code_hash, Bytecode::default()).without_code()
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
                    account.info.clone().code.unwrap().bytes_slice()
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
                let slot_present_value_vec = slot.present_value.to_le_bytes_vec();
                vec.extend(&slot_present_value_vec);

                self.set(storage_key_slice, &vec).unwrap();
            }
        }
    }
}
