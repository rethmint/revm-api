use alloy_primitives::{ Address, Bytes, StorageKey, TxKind, B256, U256 };
use prost::{ DecodeError, Message };
use revm::primitives::{ AccessListItem, TxEnv };

use crate::{ memory::ByteSliceView, v1::transaction::Transaction };

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionProto(Transaction);

impl From<Transaction> for TransactionProto {
    fn from(inner: Transaction) -> Self {
        Self(inner)
    }
}

impl TransactionProto {
    pub fn new(inner: Transaction) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> Transaction {
        self.0
    }
}

impl From<TransactionProto> for TxEnv {
    fn from(transaction: TransactionProto) -> Self {
        let transaction = transaction.into_inner();
        let transact_to = Address::from_slice(&transaction.transact_to);
        Self {
            chain_id: None,
            caller: Address::from_slice(&transaction.caller),
            gas_limit: transaction.gas_limit,
            gas_price: U256::from_be_slice(&transaction.gas_price),
            nonce: Some(transaction.nonce),
            transact_to: match transact_to {
                Address::ZERO => TxKind::Create,
                _ => TxKind::Call(transact_to),
            },
            value: U256::from_be_slice(&transaction.value),
            data: Bytes::from(transaction.data),
            gas_priority_fee: Some(U256::from_be_slice(&transaction.gas_priority_fee)),
            access_list: transaction.access_list
                .iter()
                .map(|item| {
                    AccessListItem {
                        address: Address::from_slice(&item.address),
                        storage_keys: item.storage_keys
                            .iter()
                            .map(|key| { StorageKey::from_slice(&key.value) })
                            .collect(),
                    }
                })
                .collect(),
            blob_hashes: transaction.blob_hashes
                .iter()
                .map(|hash| { B256::from_slice(hash) })
                .collect(),
            max_fee_per_blob_gas: Some(U256::from_be_slice(&transaction.max_fee_per_blob_gas)),
            authorization_list: todo!(),
        }
    }
}

impl TryFrom<ByteSliceView> for TxEnv {
    type Error = DecodeError;
    fn try_from(value: ByteSliceView) -> Result<Self, Self::Error> {
        let tx_bytes = value.read().unwrap();
        Ok(TxEnv::from(TransactionProto::from(Transaction::decode(tx_bytes).unwrap())))
    }
}
