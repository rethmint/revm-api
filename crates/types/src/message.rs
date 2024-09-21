// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

//pub const CORE_CODE_ADDRESS: Address = 1;
//pub fn genesis_address() -> Address {
//    CORE_CODE_ADDRESS
//}

use revm_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{
    Accounts, CosmosMessage, CosmosMessages, EntryFunction, GasUsageSet, JsonEvent, JsonEvents,
    StakingChangeSet, WriteSet,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Sender addresses.
    senders: Vec<Address>,
    /// The message script to execute.
    payload: MessagePayload,
}

impl Message {
    /// Create a new `Message` with a payload.
    ///
    /// It can be either to publish a module, to execute a script
    pub fn new(senders: Vec<Address>, payload: MessagePayload) -> Self {
        Message { senders, payload }
    }

    pub fn execute(senders: Vec<Address>, entry_function: EntryFunction) -> Self {
        Message {
            senders,
            payload: MessagePayload::Execute(entry_function),
        }
    }

    pub fn into_payload(self) -> MessagePayload {
        self.payload
    }

    /// Return the sender of this message.
    pub fn senders(&self) -> &[Address] {
        &self.senders
    }

    pub fn payload(&self) -> &MessagePayload {
        &self.payload
    }

    pub fn size(&self) -> usize {
        bcs::to_bytes(&self.payload())
            .expect("Unable to serialize payload")
            .len()
            + bcs::to_bytes(self.senders())
                .expect("Unable to serialize sender")
                .len()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Executes an entry function.
    Execute(EntryFunction),
}

#[derive(Default, Debug, Clone)]
pub struct MessageOutput {
    events: JsonEvents,
    write_set: WriteSet,
    staking_change_set: StakingChangeSet,
    cosmos_messages: CosmosMessages,
    new_accounts: Accounts,
    gas_usage_set: GasUsageSet,
}

impl MessageOutput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        events: JsonEvents,
        write_set: WriteSet,
        staking_change_set: StakingChangeSet,
        cosmos_messages: CosmosMessages,
        new_accounts: Accounts,
        gas_usage_set: GasUsageSet,
    ) -> Self {
        MessageOutput {
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        }
    }

    pub fn events(&self) -> &JsonEvents {
        &self.events
    }

    pub fn write_set(&self) -> &WriteSet {
        &self.write_set
    }

    pub fn staking_change_set(&self) -> &StakingChangeSet {
        &self.staking_change_set
    }

    pub fn cosmos_messages(&self) -> &CosmosMessages {
        &self.cosmos_messages
    }

    pub fn new_accounts(&self) -> &Accounts {
        &self.new_accounts
    }

    pub fn gas_usage_set(&self) -> &GasUsageSet {
        &self.gas_usage_set
    }

    pub fn into_inner(
        self,
    ) -> (
        JsonEvents,
        WriteSet,
        StakingChangeSet,
        CosmosMessages,
        Accounts,
        GasUsageSet,
    ) {
        let Self {
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        } = self;

        (
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        )
    }
}
