use std::panic::{ catch_unwind, AssertUnwindSafe };

use k256::ecdsa::SigningKey;
use revm::{
    db::{ EmptyDB, EmptyDBTyped },
    handler::{ self, post_execution, pre_execution, PostExecutionHandler },
    inspector_handle_register,
    inspectors::NoOpInspector,
    CacheState,
    Context,
    Evm,
    EvmBuilder,
    EvmContext,
    EvmHandler,
    Handler,
    State,
};
use revm_primitives::{
    AccessList,
    AccountInfo,
    Address,
    Authorization,
    BlockEnv,
    Bytes,
    CfgEnv,
    Env,
    EnvWiring,
    EthereumWiring,
    SpecId::{ self, CANCUN },
    Transaction,
    TxEnv,
    TxKind,
    TxType,
    B256,
    U256,
};
use serde::{ Deserialize, Serialize };

use crate::{
    gstorage::GoStorage,
    BlockData,
    ByteSliceView,
    Db,
    GoApi,
    TransactionData,
    UnmanagedVector,
};
// byte slice view: golang data type
// unamangedvector: ffi safe vector data type compliants with rust's ownership and data types, for returning optional error value
pub const BLOCK: &str = "block";
pub const TRANSACTION: &str = "transaction";
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct evm_t {}

pub fn to_evm(ptr: *mut evm_t) -> Option<&'static mut Evm<EthereumWiring<&mut State, ()>>> {
    if ptr.is_null() {
        None
    } else {
        let c = unsafe { &mut *(ptr as *mut Evm<EthereumWiring<&mut State, ()>>) };
        Some(c)
    }
}

// initialize vm instance with handler
#[no_mangle]
pub extern "C" fn init_vm(
    // pre_execution: Option<&PreExecutionHandler>,
    // post_execution: Option<&PostExecutionHandler>
) -> *mut evm_t {
    let context = Context::default();
    let mut handler = EvmHandler::mainnet_with_spec(SpecId::CANCUN);
    // handler.post_execution = post_execution;
    // handler.pre_execution = pre_execution;
    let vm = Box::into_raw(Box::new(Evm::new(context, handler)));
    vm as *mut evm_t
}

#[no_mangle]
pub extern "C" fn release_vm(vm: *mut vm_t) {
    if !vm.is_null() {
        // this will free cache when it goes out of scope
        let _ = unsafe { Box::from_raw(vm as *mut Evm) };
    }
}

// TODO: make return type compatible with cosmos sdk
#[no_mangle]
pub extern "C" fn allocate_vm(
    module_cache_capacity: usize,
    script_cache_capacity: usize
) -> *mut vm_t {
    let vm = Box::into_raw(Box::new(MoveVM::new(module_cache_capacity, script_cache_capacity)));
    vm as *mut vm_t
}

// VM initializer
#[no_mangle]
pub extern "C" fn execute_evm(
    vm_ptr: *mut evm_t,
    db: Db, // -> Block Cache State from KVStore
    chain_id: u64,
    block: ByteSliceView, // -> block JSON Data
    tx: ByteSliceView // -> tx JSON Data
) {
    let mut evm = match to_evm(vm_ptr) {
        Some(vm) => { vm }
        None => {
            panic!("Failed to get VM");
        }
    };
    let block = BlockData::from_json(
        &String::from_utf8(
            block
                .read()
                .ok_or_else(|| Error::unset_arg(BLOCK))?
                .to_vec()
        )?
    );
    let tx = TransactionData::from_json(
        &String::from_utf8(
            tx
                .read()
                .ok_or_else(|| Error::unset_arg(TRANSACTION))?
                .to_vec()
        )?
    );

    let mut storage = GoStorage::new(&db);
    // @winterjihwan
    // TODO: cast storage to database with trait Database in evm
    evm.context = Context::new_with_db(db);
    evm.context.evm.inner.env.block = block;
    evm.context.evm.inner.env.tx = tx;

    evm.transact_commit();
}
