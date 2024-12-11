use std::ops::Deref;

use alloy_primitives::{ Address, B256, U256 };
use revm::primitives::BlockEnv;
use revmc::primitives::BlobExcessGasAndPrice;

use crate::{ error::BackendError, v1::block::{ self, Block } };

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockWrapper(Block);

impl From<Block> for BlockWrapper {
    fn from(inner: Block) -> Self {
        Self(inner)
    }
}

impl Deref for BlockWrapper {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockWrapper {
    pub fn new(inner: Block) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> Block {
        self.0
    }
}

impl TryFrom<BlockWrapper> for BlockEnv {
    // TODO Error for parsing
    type Error = BackendError;

    fn try_from(block: BlockWrapper) -> Result<Self, Self::Error> {
        let block = block.into_inner();
        let blob_excess_gas_and_price = block.blob_excess_gas_and_price.unwrap_or(None);
        let prevrandao = B256::from_slice(&block.prevrandao);
        Ok(Self {
            number: U256::from_be_bytes(block.number),
            coinbase: Address(block.coinbase),
            timestamp: U256::from_be_bytes(block.timestamp),
            gas_limit: U256::from_be_bytes(block.gas_limit),
            basefee: U256::from_be_bytes(block.basefee),
            difficulty: U256::from_be_bytes(block.difficulty),
            prevrandao: match prevrandao {
                B256::ZERO => None,
                _ => Some(prevrandao),
            },
            blob_excess_gas_and_price: BlobExcessGasAndPrice {
                excess_blob_gas: blob_excess_gas_and_price.excess_blob_gas,
                blob_gasprice: u128::from_be_bytes(
                    blob_excess_gas_and_price.blob_gasprice
                        .try_into()
                        .map_err(|err| "Overflow the gas price")
                ),
            },
        })
    }
}
