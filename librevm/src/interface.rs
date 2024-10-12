use alloy_primitives::B256;
use flatbuffer_types::{
    block::Block,
    result::{
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
    },
    transaction::Transaction,
};
use revm::{
    handler::register::EvmHandler,
    primitives::{
        Address,
        BlobExcessGasAndPrice,
        BlockEnv,
        Bytes,
        ExecutionResult,
        HaltReason,
        OutOfGasError,
        SpecId,
        SuccessReason,
        TxEnv,
        TxKind,
        U256,
    },
    Context,
    Evm,
};

use crate::{ gstorage::GoStorage, set_error, ByteSliceView, Db, UnmanagedVector };

// byte slice view: golang data type
// unamangedvector: ffi safe vector data type compliants with rust's ownership and data types, for returning optional error value
pub const BLOCK: &str = "block";
pub const TRANSACTION: &str = "transaction";
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct evm_t {}

pub fn to_evm<'a>(ptr: *mut evm_t) -> Option<&'a mut Evm<'a, (), GoStorage<'a>>> {
    if ptr.is_null() {
        None
    } else {
        let evm = unsafe { &mut *(ptr as *mut Evm<'a, (), GoStorage<'a>>) };
        Some(evm)
    }
}
// initialize vm instance with handler
#[no_mangle]
pub extern "C" fn init_vm(
    // [] handler type -> validation / pre-execution / post-execution
    // GoApi -> api based on cosmos sdk
) -> *mut evm_t {
    let db = Db::default();
    let gstorage = GoStorage::new(&db);
    let context = Context::<(), GoStorage>::new_with_db(gstorage);
    let handler = EvmHandler::mainnet_with_spec(SpecId::CANCUN);
    // handler.post_execution = post_execution;
    // handler.pre_execution = pre_execution;
    let vm = Box::into_raw(Box::new(Evm::new(context, handler)));
    vm as *mut evm_t
}

#[no_mangle]
pub extern "C" fn release_vm(vm: *mut evm_t) {
    if !vm.is_null() {
        // this will free cache when it goes out of scope
        let _ = unsafe { Box::from_raw(vm as *mut Evm<'static, (), GoStorage<'static>>) };
    }
}

// VM initializer
#[no_mangle]
pub extern "C" fn execute_tx(
    vm_ptr: *mut evm_t,
    db: Db, // -> Block Cache State from KVStore
    block: ByteSliceView,
    tx: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>
) -> UnmanagedVector {
    let evm = match to_evm(vm_ptr) {
        Some(vm) => vm,
        None => {
            panic!("Failed to get VM");
        }
    };
    let go_storage = GoStorage::new(&db);
    evm.context = Context::new_with_db(go_storage);
    set_evm_env(evm, block, tx);

    let result = evm.transact_commit();

    let data = match result {
        Ok(res) => build_flat_buffer(res),
        Err(err) => {
            set_error(err, errmsg);
            Vec::new()
        }
    };

    UnmanagedVector::new(Some(data))
}

#[no_mangle]
pub extern "C" fn query(
    vm_ptr: *mut evm_t,
    db: Db, // -> Block Cache State from KVStore
    block: ByteSliceView,
    tx: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>
) -> UnmanagedVector {
    let evm = match to_evm(vm_ptr) {
        Some(vm) => vm,
        None => {
            panic!("Failed to get VM");
        }
    };
    let db = GoStorage::new(&db);
    evm.context = Context::new_with_db(db);
    set_evm_env(evm, block, tx);
    // transact without state commit
    let result = evm.transact();
    let data = match result {
        Ok(res) => build_flat_buffer(res.result),
        Err(err) => {
            set_error(err, errmsg);
            Vec::new()
        }
    };

    UnmanagedVector::new(Some(data))
}

fn build_flat_buffer(result: ExecutionResult) -> Vec<u8> {
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
            let halt_reason = match reason {
                HaltReason::OutOfGas(out_of_gas_error) =>
                    match out_of_gas_error {
                        OutOfGasError::Basic => HaltReasonEnum::OutOfGasBasic,
                        OutOfGasError::MemoryLimit => { HaltReasonEnum::OutOfGasMemoryLimit }
                        OutOfGasError::Memory => HaltReasonEnum::OutOfGasMemory,
                        OutOfGasError::Precompile => { HaltReasonEnum::OutOfGasPrecompile }
                        OutOfGasError::InvalidOperand => { HaltReasonEnum::OutOfGasInvalidOperand }
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
    let finished_data = builder.finished_data();
    let mut res = Vec::new();
    res.extend_from_slice(finished_data);
    res
}

fn set_evm_env(evm: &mut Evm<(), GoStorage>, block: ByteSliceView, tx: ByteSliceView) {
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
        nonce: None,
        access_list: Vec::new(),
        blob_hashes: Vec::new(),
        max_fee_per_blob_gas: None,
        authorization_list: None,
    };
    evm.context.evm.inner.env.block = block_env;
    evm.context.evm.inner.env.tx = tx_env;
}
