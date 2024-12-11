use alloy_primitives::{ Address, Bytes, StorageKey, TxKind, U256 };
use revm::primitives::{ AccessListItem, Authorization, TxEnv };

use crate::{ error::BackendError, v1::transaction::Transaction };

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TransactionWrapper(Transaction);

impl From<Transaction> for TransactionWrapper {
    fn from(inner: Transaction) -> Self {
        Self(inner)
    }
}

impl TransactionWrapper {
    pub fn new(inner: Transaction) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> Transaction {
        self.0
    }
}

impl TryFrom<TransactionWrapper> for TxEnv {
    //TODO: make error for parsing
    type Error = BackendError;

    fn try_from(transaction: TransactionWrapper) -> Result<Self, Self::Error> {
        let transaction = transaction.into_inner();
        let transact_to = Address::from_slice(&transaction.transact_to);
        Ok(Self {
            chain_id: None,
            caller: Address::from_slice(&transaction.caller),
            gas_limit: transaction.gas_limit,
            gas_price: U256::from_be_byte(transaction.gas_price),
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
                        address: Address(item.address),
                        storage_keys: item.storage_keys
                            .iter()
                            .map(|key| { StorageKey::from_slice(key) })
                            .collect(),
                    }
                })
                .collect(),
            blob_hashes: transaction.blob_hashes
                .iter()
                .map(|hash| { U256::from_be_slice(hash) })
                .collect(),
            max_fee_per_blob_gas: Some(U256::from_be_slice(&transaction.max_fee_per_blob_gas)),
            authorization_list: todo!(),
        })
    }
}
