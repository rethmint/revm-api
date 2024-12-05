use alloy_primitives::{Address, Bytes, FixedBytes, TxKind, B256, U256};
use flatbuffer_types::{block::Block, transaction::Transaction};

use revm::{
    primitives::{AccessList, AccessListItem, BlobExcessGasAndPrice, BlockEnv, TxEnv},
    Evm,
};

use crate::{memory::ByteSliceView, states::GoStorage};

pub fn set_evm_env<EXT>(evm: &mut Evm<EXT, GoStorage>, block: ByteSliceView, tx: ByteSliceView) {
    let block_bytes = block.read().unwrap();
    let block = flatbuffers::root::<Block>(block_bytes).unwrap();
    let block_env = BlockEnv {
        number: U256::from_be_slice(block.number().unwrap().bytes()),
        coinbase: Address::from_slice(block.coinbase().unwrap().bytes()),
        timestamp: U256::from_be_slice(block.timestamp().unwrap().bytes()),
        gas_limit: U256::from_be_slice(block.gas_limit().unwrap().bytes()),
        basefee: U256::from_be_slice(block.basefee().unwrap().bytes()),
        difficulty: U256::ZERO,
        prevrandao: Some(B256::ZERO),
        blob_excess_gas_and_price: Some(BlobExcessGasAndPrice::new(0)),
    };

    let tx_bytes = tx.read().unwrap();
    let tx = flatbuffers::root::<Transaction>(tx_bytes).unwrap();

    let tx_env = TxEnv {
        caller: Address::from_slice(tx.caller().unwrap().bytes()),
        gas_price: U256::from_be_slice(tx.gas_price().unwrap().bytes()),
        gas_limit: tx.gas_limit(),
        value: U256::from_be_slice(tx.value().unwrap().bytes()),
        data: Bytes::from(tx.data().unwrap().bytes().to_vec()),
        chain_id: None,
        gas_priority_fee: Some(U256::from_be_slice(tx.gas_priority_fee().unwrap().bytes())),
        transact_to: match Address::from_slice(tx.transact_to().unwrap().bytes()) {
            Address::ZERO => TxKind::Create,
            address => TxKind::Call(address),
        },
        nonce: Some(tx.nonce()),
        access_list: AccessList::from(
            tx.access_list()
                .unwrap()
                .into_iter()
                .filter_map(|al| {
                    al.address().and_then(|address| {
                        al.storage_key().map(|storage_keys| AccessListItem {
                            address: Address::from_slice(address.bytes()),
                            storage_keys: storage_keys
                                .into_iter()
                                .map(|sk| {
                                    FixedBytes::<32>::try_from(sk.value().unwrap().bytes()).unwrap()
                                })
                                .collect(),
                        })
                    })
                })
                .collect::<Vec<AccessListItem>>(),
        )
        .to_vec(),
        blob_hashes: Vec::new(),
        max_fee_per_blob_gas: None,
        authorization_list: None,
    };
    evm.context.evm.inner.env.block = block_env;
    evm.context.evm.inner.env.tx = tx_env;
}
