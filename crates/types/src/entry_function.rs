use ethers_core::types::H256;
use revm_primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntryFunction {
    ca: Address,
    function: H256,
    args: Vec<Vec<u8>>,

    // whether the args are json encoded
    is_json: bool,
}

impl EntryFunction {
    pub fn new(ca: Address, function: H256, args: Vec<Vec<u8>>, is_json: bool) -> Self {
        EntryFunction {
            ca,
            function,
            args,
            is_json,
        }
    }

    pub fn ca(&self) -> &Address {
        &self.ca
    }

    pub fn function(&self) -> &H256 {
        &self.function
    }

    pub fn args(&self) -> &[Vec<u8>] {
        &self.args
    }

    pub fn into_inner(self) -> (Address, H256, Vec<Vec<u8>>) {
        (self.ca, self.function, self.args)
    }

    pub fn is_json(&self) -> bool {
        self.is_json
    }
}
