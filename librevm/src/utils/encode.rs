use alloy_primitives::Address;
use flatbuffer_types::result::{
    finish_evm_result_buffer,
    EvmResult,
    EvmResultArgs,
    ExResult,
    Halt,
    HaltArgs,
    HaltReasonEnum,
    Log,
    LogArgs,
    LogData,
    LogDataArgs,
    Revert,
    RevertArgs,
    Success,
    SuccessArgs,
    SuccessReasonEnum,
    Topic,
    TopicArgs,
};
use revm::primitives::{ ExecutionResult, HaltReason, OutOfGasError, SuccessReason };

pub fn build_flat_buffer(result: ExecutionResult) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(300);
    let args = match result {
        ExecutionResult::Success { reason, gas_used, gas_refunded, logs, output } => {
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
                logs_buffer.push(
                    Log::create(
                        &mut builder,
                        &(LogArgs {
                            address: Some(address),
                            data: Some(data),
                        })
                    )
                );
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
                })
            );

            EvmResult::create(
                &mut builder,
                &(EvmResultArgs {
                    result_type: ExResult::Success,
                    result: Some(success_offset.as_union_value()),
                })
            )
        }
        ExecutionResult::Revert { gas_used, output } => {
            let output_offset = builder.create_vector(output.as_ref());
            let revert_offset = Revert::create(
                &mut builder,
                &(RevertArgs {
                    gas_used,
                    output: Some(output_offset),
                })
            );

            EvmResult::create(
                &mut builder,
                &(EvmResultArgs {
                    result_type: ExResult::Revert,
                    result: Some(revert_offset.as_union_value()),
                })
            )
        }
        ExecutionResult::Halt { reason, gas_used } => {
            println!("Reason: {:?}", reason);
            println!("Gas Used: {:?}", gas_used);
            let halt_reason = match reason {
                HaltReason::OutOfGas(out_of_gas_error) =>
                    match out_of_gas_error {
                        OutOfGasError::Basic => HaltReasonEnum::OutOfGasBasic,
                        OutOfGasError::MemoryLimit => HaltReasonEnum::OutOfGasMemoryLimit,
                        OutOfGasError::Memory => HaltReasonEnum::OutOfGasMemory,
                        OutOfGasError::Precompile => HaltReasonEnum::OutOfGasPrecompile,
                        OutOfGasError::InvalidOperand => HaltReasonEnum::OutOfGasInvalidOperand,
                    }
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
                })
            );
            EvmResult::create(
                &mut builder,
                &(EvmResultArgs {
                    result_type: ExResult::Halt,
                    result: Some(halt_offset.as_union_value()),
                })
            )
        }
    };
    finish_evm_result_buffer(&mut builder, args);
    builder.finished_data().to_vec()
}

pub fn ivec_to_u64(ivec: &sled::IVec) -> Option<u64> {
    ivec.as_ref().try_into().ok().map(u64::from_be_bytes)
}
