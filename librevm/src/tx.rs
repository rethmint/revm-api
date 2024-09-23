use revm_primitives::{
    AccessListItem, Address, AuthorizationList, Bytes, Transaction, TxKind, B256, GAS_PER_BLOB,
    U256,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub kind: TxKind,
    pub value: U256,
    pub data: Bytes,
    pub nonce: u64,
    pub chain_id: Option<u64>,
    pub access_list: Vec<AccessListItem>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub blob_hashes: Vec<B256>,
    pub max_fee_per_blob_gas: Option<U256>,
    pub authorization_list: Option<AuthorizationList>,
}

impl Transaction for TransactionData {
    fn caller(&self) -> &Address {
        &self.caller
    }

    fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    fn gas_price(&self) -> &U256 {
        &self.gas_price
    }

    fn kind(&self) -> TxKind {
        self.kind
    }

    fn value(&self) -> &U256 {
        &self.value
    }

    fn data(&self) -> &Bytes {
        &self.data
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn chain_id(&self) -> Option<u64> {
        self.chain_id
    }

    fn access_list(&self) -> &[AccessListItem] {
        &self.access_list
    }

    fn max_priority_fee_per_gas(&self) -> Option<&U256> {
        self.max_priority_fee_per_gas.as_ref()
    }

    fn blob_hashes(&self) -> &[B256] {
        &self.blob_hashes
    }

    fn max_fee_per_blob_gas(&self) -> Option<&U256> {
        self.max_fee_per_blob_gas.as_ref()
    }

    fn authorization_list(&self) -> Option<&AuthorizationList> {
        self.authorization_list.as_ref()
    }

    fn get_total_blob_gas(&self) -> u64 {
        GAS_PER_BLOB * (self.blob_hashes.len() as u64)
    }
}

impl TransactionData {
    pub fn from_json(json_str: &str) -> Self {
        match serde_json::from_str(json_str) {
            Ok(tx) => tx,
            Err(e) => panic!("Failed to parse JSON: {}", e),
        }
    }
}
