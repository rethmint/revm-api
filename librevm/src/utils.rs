use alloy_primitives::{Address, Bytes, FixedBytes, TxKind, B256, U256};
use flatbuffer_types::{
    block::Block,
    result::{
        finish_evm_result_buffer, EvmResult, EvmResultArgs, ExResult, Halt, HaltArgs,
        HaltReasonEnum, Log, LogArgs, LogData, LogDataArgs, Revert, RevertArgs, Success,
        SuccessArgs, SuccessReasonEnum, Topic, TopicArgs,
    },
    transaction::Transaction,
};
use revm::{
    primitives::{
        AccessList, AccessListItem, BlobExcessGasAndPrice, BlockEnv, ExecutionResult, HaltReason,
        OutOfGasError, SuccessReason, TxEnv,
    },
    Evm,
};
use std::path::PathBuf;

use crate::{gstorage::GoStorage, memory::ByteSliceView};

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

pub fn build_flat_buffer(result: ExecutionResult) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(300);
    let args = match result {
        ExecutionResult::Success {
            reason,
            gas_used,
            gas_refunded,
            logs,
            output,
        } => {
            let reason = match reason {
                SuccessReason::Stop => SuccessReasonEnum::Stop,
                SuccessReason::Return => SuccessReasonEnum::Return,
                SuccessReason::SelfDestruct => SuccessReasonEnum::SelfDestruct,
                SuccessReason::EofReturnContract => SuccessReasonEnum::EofReturnContract,
            };
            let mut logs_buffer = Vec::new();
            for log in logs.iter() {
                let mut topics_buffer = Vec::new();
                for topic in log.topics() {
                    let topic_args = TopicArgs {
                        value: Some(builder.create_vector(topic.as_ref())),
                    };
                    topics_buffer.push(Topic::create(&mut builder, &topic_args));
                }
                let log_data_args = LogDataArgs {
                    topics: Some(builder.create_vector(&topics_buffer)),
                    data: Some(builder.create_vector(log.data.data.as_ref())),
                };
                let data = LogData::create(&mut builder, &log_data_args);
                let address = builder.create_vector(log.address.as_ref());
                logs_buffer.push(Log::create(
                    &mut builder,
                    &(LogArgs {
                        address: Some(address),
                        data: Some(data),
                    }),
                ));
            }
            let logs = Some(builder.create_vector(&logs_buffer));

            let deployed_address = output.address().unwrap_or(&Address::ZERO).to_vec();
            let deployed_address_vec = Some(builder.create_vector(&deployed_address));
            let output_data_vec = Some(builder.create_vector(output.data()));

            let success_offset = Success::create(
                &mut builder,
                &(SuccessArgs {
                    reason,
                    gas_used,
                    gas_refunded,
                    logs,
                    deployed_address: deployed_address_vec,
                    output: output_data_vec,
                }),
            );

            EvmResult::create(
                &mut builder,
                &(EvmResultArgs {
                    result_type: ExResult::Success,
                    result: Some(success_offset.as_union_value()),
                }),
            )
        }
        ExecutionResult::Revert { gas_used, output } => {
            let output_offset = builder.create_vector(output.as_ref());
            let revert_offset = Revert::create(
                &mut builder,
                &(RevertArgs {
                    gas_used,
                    output: Some(output_offset),
                }),
            );

            EvmResult::create(
                &mut builder,
                &(EvmResultArgs {
                    result_type: ExResult::Revert,
                    result: Some(revert_offset.as_union_value()),
                }),
            )
        }
        ExecutionResult::Halt { reason, gas_used } => {
            println!("Reason: {:?}", reason);
            println!("Gas Used: {:?}", gas_used);
            let halt_reason = match reason {
                HaltReason::OutOfGas(out_of_gas_error) => match out_of_gas_error {
                    OutOfGasError::Basic => HaltReasonEnum::OutOfGasBasic,
                    OutOfGasError::MemoryLimit => HaltReasonEnum::OutOfGasMemoryLimit,
                    OutOfGasError::Memory => HaltReasonEnum::OutOfGasMemory,
                    OutOfGasError::Precompile => HaltReasonEnum::OutOfGasPrecompile,
                    OutOfGasError::InvalidOperand => HaltReasonEnum::OutOfGasInvalidOperand,
                },
                HaltReason::OpcodeNotFound => HaltReasonEnum::OpcodeNotFound,
                HaltReason::InvalidFEOpcode => HaltReasonEnum::InvalidFEOpcode,
                HaltReason::InvalidJump => HaltReasonEnum::InvalidJump,
                HaltReason::NotActivated => HaltReasonEnum::NotActivated,
                HaltReason::StackUnderflow => HaltReasonEnum::StackUnderflow,
                HaltReason::StackOverflow => HaltReasonEnum::StackOverflow,
                HaltReason::OutOfOffset => HaltReasonEnum::OutOfOffset,
                HaltReason::CreateCollision => HaltReasonEnum::CreateCollision,
                HaltReason::PrecompileError => HaltReasonEnum::PrecompileError,
                HaltReason::NonceOverflow => HaltReasonEnum::NonceOverflow,
                HaltReason::CreateContractSizeLimit => HaltReasonEnum::CreateContractSizeLimit,
                HaltReason::CreateContractStartingWithEF => {
                    HaltReasonEnum::CreateContractStartingWithEF
                }
                HaltReason::CreateInitCodeSizeLimit => HaltReasonEnum::CreateInitCodeSizeLimit,
                HaltReason::OverflowPayment => HaltReasonEnum::OverflowPayment,
                HaltReason::StateChangeDuringStaticCall => {
                    HaltReasonEnum::StateChangeDuringStaticCall
                }
                HaltReason::CallNotAllowedInsideStatic => {
                    HaltReasonEnum::CallNotAllowedInsideStatic
                }
                HaltReason::OutOfFunds => HaltReasonEnum::OutOfFunds,
                HaltReason::CallTooDeep => HaltReasonEnum::CallTooDeep,
                HaltReason::EofAuxDataOverflow => HaltReasonEnum::EofAuxDataOverflow,
                HaltReason::EofAuxDataTooSmall => HaltReasonEnum::EofAuxDataTooSmall,
                HaltReason::EOFFunctionStackOverflow => HaltReasonEnum::EOFFunctionStackOverflow,
                HaltReason::InvalidEXTCALLTarget => HaltReasonEnum::InvalidEXTCALLTarget,
            };
            let halt_offset = Halt::create(
                &mut builder,
                &(HaltArgs {
                    gas_used,
                    reason: halt_reason,
                }),
            );
            EvmResult::create(
                &mut builder,
                &(EvmResultArgs {
                    result_type: ExResult::Halt,
                    result: Some(halt_offset.as_union_value()),
                }),
            )
        }
    };
    finish_evm_result_buffer(&mut builder, args);
    builder.finished_data().to_vec()
}

pub fn ivec_to_u64(ivec: &sled::IVec) -> Option<u64> {
    ivec.as_ref().try_into().ok().map(u64::from_be_bytes)
}

pub fn ivec_to_pathbuf(ivec: &sled::IVec) -> Option<PathBuf> {
    String::from_utf8(ivec.to_vec()).ok().map(PathBuf::from)
}
