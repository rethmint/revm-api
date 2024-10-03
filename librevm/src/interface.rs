use revm::{Context, Evm, EvmHandler};
use revm_primitives::{EthereumWiring, ExecutionResult, HaltReason, SpecId};
use serde::{Deserialize, Serialize};
use types::{BlockEnvSansEip4844, TxEnvSansEip4844};

use crate::{gstorage::GoStorage, ByteSliceView, Db, UnmanagedVector};
// byte slice view: golang data type
// unamangedvector: ffi safe vector data type compliants with rust's ownership and data types, for returning optional error value
pub const BLOCK: &str = "block";
pub const TRANSACTION: &str = "transaction";
enum ResultId {
    Success,
    Revert,
    Halt,
    Error,
}
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct evm_t {}

pub fn to_evm<'a>(ptr: *mut evm_t) -> Option<&'a mut Evm<'a, EthereumWiring<GoStorage<'a>, ()>>> {
    if ptr.is_null() {
        None
    } else {
        //let vm: *mut Evm<'_, EthereumWiring<EmptyDBTyped<Infallible>, ()>>
        let evm = unsafe { &mut *(ptr as *mut Evm<'a, EthereumWiring<GoStorage<'a>, ()>>) };
        Some(evm)
    }
}

// initialize vm instance with handler
#[no_mangle]
pub extern "C" fn init_vm(// [] handler type -> validation / pre-execution / post-execution
    // GoApi -> api based on cosmos sdk
) -> *mut evm_t {
    let db = Db::default();
    let gstorage = GoStorage::new(&db);
    let context = Context::<EthereumWiring<GoStorage, ()>>::new_with_db(gstorage);
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
        let _ = unsafe {
            Box::from_raw(vm as *mut Evm<'static, EthereumWiring<GoStorage<'static>, ()>>)
        };
    }
}

// VM initializer
#[no_mangle]
pub extern "C" fn execute_tx(
    vm_ptr: *mut evm_t,
    db: Db,               // -> Block Cache State from KVStore
    block: ByteSliceView, // -> block JSON Data
    tx: ByteSliceView,    // -> tx JSON Data
                          // errmsg: Option<&mut UnmanagedVector>
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

    let result = evm.transact_commit();

    let data = match result {
        Ok(res) => handle_id(res),
        Err(_err) => {
            // let msg = err.to_string().into();
            // set_error(err, errmsg);
            vec![ResultId::Error as u8]
        }
    };
    UnmanagedVector::new(Some(data))
}

#[no_mangle]
pub extern "C" fn query(
    vm_ptr: *mut evm_t,
    db: Db,               // -> Block Cache State from KVStore
    block: ByteSliceView, // -> block JSON Data
    tx: ByteSliceView,    // -> tx JSON Data
                          // errmsg: Option<&mut UnmanagedVector>
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
        Ok(res) => handle_id(res.result),
        Err(_err) => {
            // let msg = err.to_string().into();
            // set_error(err, errmsg);
            vec![ResultId::Error as u8]
        }
    };
    UnmanagedVector::new(Some(data))
}

fn handle_id(result: ExecutionResult<HaltReason>) -> Vec<u8> {
    let mut result = match result {
        ExecutionResult::Success {
            reason: _,
            gas_used: _,
            gas_refunded: _,
            logs: _,
            output: _,
        } => vec![ResultId::Success as u8],
        ExecutionResult::Revert {
            gas_used: _,
            output: _,
        } => vec![ResultId::Revert as u8],
        ExecutionResult::Halt {
            reason: _,
            gas_used: _,
        } => vec![ResultId::Halt as u8],
    };
    let mut data = serde_json::to_vec(&result).unwrap();
    result.append(&mut data);
    result
}

fn set_evm_env(
    evm: &mut Evm<'_, EthereumWiring<GoStorage<'_>, ()>>,
    block: ByteSliceView,
    tx: ByteSliceView,
) {
    let block: BlockEnvSansEip4844 = serde_json::from_str(
        &String::from_utf8({
            let a = block
                .read()
                .unwrap()
                //.ok_or_else(|| Error::unset_arg(BLOCK))?
                .to_vec();
            //println!("a: {a:#?}");
            a
        })
        .expect("BlockEnvSansEip4844 parse: string from utf 8 failed"),
    )
    .expect("BlockEnvSansEip4844 parse: serde from str failed for");

    let tx: TxEnvSansEip4844 = serde_json::from_str(
        &String::from_utf8({
            let b = tx
                .read()
                .unwrap()
                //.ok_or_else(|| Error::unset_arg(BLOCK))?
                .to_vec();
            //println!("b: {b:#?}");
            b
        })
        .expect("TxEnvSansEip4844 parse: string from utf 8 failed"),
    )
    .expect("TxEnvSansEip4844 parse: string from str failed");

    evm.context.evm.inner.env.block = block.into();
    evm.context.evm.inner.env.tx = tx.into();
}

#[derive(Serialize, Deserialize, Debug)]
struct MockTx {
    from: String,
    to: String,
    value: String,
}

fn deserialize_json<T>(bytes: ByteSliceView) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    // Convert ByteSliceView to Vec<u8>
    let byte_data = bytes.read().unwrap().to_vec();

    // Convert Vec<u8> to String
    let json_str = match String::from_utf8(byte_data) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to convert bytes to string: {}", e)),
    };

    // Deserialize the JSON string into the target type T
    let deserialized_data: T = match serde_json::from_str(&json_str) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to deserialize JSON: {}", e)),
    };

    Ok(deserialized_data)
}

#[no_mangle]
pub extern "C" fn deserialize_unit_test(tx_view: ByteSliceView) {
    let tx: MockTx = deserialize_json::<MockTx>(tx_view).unwrap();

    println!("Mock tx: {:?}", tx);
}

#[no_mangle]
pub extern "C" fn deserialize_block_env(tx_view: ByteSliceView) {
    let tx: BlockEnvSansEip4844 = deserialize_json::<BlockEnvSansEip4844>(tx_view).unwrap();

    println!("BlockEnvSansEip4844: {:?}", tx);
}
