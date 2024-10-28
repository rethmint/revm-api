// KVStore
// ACCOUNT_PREFIX(B1) + {address(B20)} => ACCOUNT INFO {balance(B64)(0) | nonce(B256)(1) | code_hash(B256)(2)}
// CODE_PREFIX(B1) + {code_hash(B32)} => vm bytecode
// STORAGE_PREFIX(B1) + {address(B20)} + {index(B32)} => [32]byte(value)
// BLOCK_PREFIX(B1) + block_num(B8) => block_hash

use alloy_primitives::{ Address, B256, U256 };
use state::{ AccountInfo, Bytecode };

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

// compress account info data with bigedien order, not included bytecode
pub fn compress_account_info(info: AccountInfo) -> Vec<u8> {
    let mut vec = Vec::with_capacity(72);

    let balance_be_bytes = info.balance.to_be_bytes_vec();
    vec.extend(&balance_be_bytes);

    let nonce_be_bytes = info.nonce.to_be_bytes();
    vec.extend_from_slice(&nonce_be_bytes);

    vec.extend(info.code_hash.to_vec());

    vec
}
// return Account info with no code
pub fn parse_account_info(value: Vec<u8>) -> AccountInfo {
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
