use alloy_primitives::{ Address, Bytes, FixedBytes, TxKind, B256, U256 };
use revm::{
    primitives::{ AccessList, AccessListItem, BlobExcessGasAndPrice, BlockEnv, TxEnv },
    Evm,
};

use crate::{
    memory::ByteSliceView,
    states::GoCacheDB,
    v1::{ block::Block, transaction::Transaction },
};

pub fn set_evm_env<EXT>(evm: &mut Evm<EXT, GoCacheDB>, block: ByteSliceView, tx: ByteSliceView) {
    let block_bytes = block.read().unwrap();
    let block_env: BlockEnv = Block::from(block_bytes).into();

    let tx_bytes = tx.read().unwrap();
    let tx: TxEnv = Transaction::from(tx_bytes).into();
    evm.context.evm.inner.env.block = block_env;
    evm.context.evm.inner.env.tx = tx_env;
}
