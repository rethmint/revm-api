use revm_primitives::{Address, BlockEnv, Bytes, TxEnv, TxKind, U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockEnvSansEip4844 {
    pub number: U256,
    pub coinbase: Address,
    pub timestamp: U256,
    pub gas_limit: U256,
    pub basefee: U256,
}

impl From<BlockEnvSansEip4844> for BlockEnv {
    fn from(block: BlockEnvSansEip4844) -> Self {
        BlockEnv {
            number: block.number,
            coinbase: block.coinbase,
            timestamp: block.timestamp,
            gas_limit: block.gas_limit,
            basefee: block.basefee,
            difficulty: U256::from(0),
            prevrandao: None,
            blob_excess_gas_and_price: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxEnvSansEip4844 {
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub transact_to: TxKind,
    pub value: U256,
    pub data: Bytes,
    pub nonce: u64,
    pub chain_id: Option<u64>,
    pub gas_priority_fee: Option<U256>,
}

impl From<TxEnvSansEip4844> for TxEnv {
    fn from(tx: TxEnvSansEip4844) -> Self {
        TxEnv {
            caller: tx.caller,
            gas_limit: tx.gas_limit,
            gas_price: tx.gas_price,
            transact_to: tx.transact_to,
            value: tx.value,
            data: tx.data,
            nonce: tx.nonce,
            chain_id: tx.chain_id,
            gas_priority_fee: tx.gas_priority_fee,
            access_list: Vec::new(),
            blob_hashes: Vec::new(),
            max_fee_per_blob_gas: None,
            authorization_list: None,
        }
    }
}
