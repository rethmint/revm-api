// KVStore
// ACCOUNT_PREFIX(B1) + {address(B20)} => ACCOUNT INFO {balance(B64)(0) | nonce(B256)(1) | code_hash(B256)(2)}
// CODE_PREFIX(B1) + {code_hash(B32)} => vm bytecode
// STORAGE_PREFIX(B1) + {address(B20)} + {index(B32)} => [32]byte(value)
// BLOCK_PREFIX(B1) + block_num(B8) => block_hash

use alloy_primitives::{Address, B256, U256};
use revm::primitives::{AccountInfo, Bytecode};

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

pub enum EvmStoreKey {
    Account(Address),
    Code(CodeHash),
    Storage(Address, StorageIndex),
    Block(BlockNum),
}

impl EvmStoreKey {
    pub fn key(self) -> Vec<u8> {
        match self {
            Self::Account(addr) => {
                let mut result: Vec<u8> = vec![EvmStoreKeyPrefix::Account.into()];
                result.append(&mut addr.to_vec());
                result
            }
            Self::Code(code_hash) => {
                let mut result = vec![EvmStoreKeyPrefix::Code.into()];
                result.append(&mut code_hash.to_vec());
                result
            }
            Self::Storage(addr, slot) => {
                let mut result = vec![EvmStoreKeyPrefix::Storage.into()];
                result.append(&mut addr.to_vec());
                result.append(&mut slot.to_be_bytes::<32>().to_vec());
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

