use revm_primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosMessages(Vec<CosmosMessage>);

impl CosmosMessages {
    pub fn new(map: Vec<CosmosMessage>) -> Self {
        Self(map)
    }

    pub fn inner(&self) -> &Vec<CosmosMessage> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<CosmosMessage> {
        self.0
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CosmosMessage {
    Move(MoveMessage),
    Staking(StakingMessage),
    Distribution(DistributionMessage),
    IBC(IBCMessage),
    Stargate(StargateMessage),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct StargateMessage {
    pub sender: Address,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveMessage {
    Execute {
        sender: Address,
        module_address: Address,
        module_name: String,
        function_name: String,
        type_args: Vec<String>,
        args: Vec<Vec<u8>>,
        is_json: bool,
    },
    Script {
        sender: Address,
        code_bytes: Vec<u8>,
        type_args: Vec<String>,
        args: Vec<Vec<u8>>,
        is_json: bool,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum StakingMessage {
    Delegate {
        delegator_address: Address,
        validator_address: String,
        amount: CosmosCoin,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum DistributionMessage {
    FundCommunityPool {
        sender_address: Address,
        amount: CosmosCoin,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum IBCMessage {
    Transfer {
        source_port: String,
        source_channel: String,
        token: CosmosCoin,
        sender: Address,
        receiver: String,
        timeout_height: IBCHeight,
        timeout_timestamp: u64,
        memo: String,
    },
    NFTTransfer {
        source_port: String,
        source_channel: String,
        collection: Address,
        token_ids: Vec<String>,
        sender: Address,
        receiver: String,
        timeout_height: IBCHeight,
        timeout_timestamp: u64,
        memo: String,
    },
    PayFee {
        fee: IBCFee,
        source_port: String,
        source_channel: String,
        signer: Address,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosCoin {
    pub metadata: Address,
    pub amount: u64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct IBCHeight {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct IBCFee {
    pub recv_fee: CosmosCoin,
    pub ack_fee: CosmosCoin,
    pub timeout_fee: CosmosCoin,
}
