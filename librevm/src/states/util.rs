use alloy_primitives::{B256, U256};
use revm::primitives::{AccountInfo, Bytecode};

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
