use revm_primitives::Block;

use crate::{ Address, BlobExcessGasAndPrice, B256, U256 };
use std::str::FromStr;
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockData {
    pub number: U256,
    pub coinbase: Address,
    pub timestamp: U256,
    pub gas_limit: U256,
    pub basefee: U256,
    pub difficulty: U256,
    pub prevrandao: Option<B256>,
    pub blob_excess_gas_and_price: Option<BlobExcessGasAndPrice>,
}

impl BlockData {
    pub fn from_json(json_str: &str) -> Self {
        match serde_json::from_str(json_str) {
            Ok(tx) => tx,
            Err(e) => panic!("Failed to parse JSON: {}", e),
        }
    }
}

impl Block for BlockStruct {
    fn number(&self) -> &U256 {
        &self.number
    }

    fn coinbase(&self) -> &Address {
        &self.coinbase
    }

    fn timestamp(&self) -> &U256 {
        &self.timestamp
    }

    fn gas_limit(&self) -> &U256 {
        &self.gas_limit
    }

    fn basefee(&self) -> &U256 {
        &self.basefee
    }

    fn difficulty(&self) -> &U256 {
        &self.difficulty
    }

    fn prevrandao(&self) -> Option<&B256> {
        self.prevrandao.as_ref()
    }

    fn blob_excess_gas_and_price(&self) -> Option<&BlobExcessGasAndPrice> {
        self.blob_excess_gas_and_price.as_ref()
    }
}
