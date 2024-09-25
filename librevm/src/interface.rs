use revm::{Context, Evm, EvmHandler};
use revm_primitives::{BlockEnv, EthereumWiring, SpecId, TxEnv};

use crate::{gstorage::GoStorage, ByteSliceView, Db, UnmanagedVector};
// byte slice view: golang data type
// unamangedvector: ffi safe vector data type compliants with rust's ownership and data types, for returning optional error value
pub const BLOCK: &str = "block";
pub const TRANSACTION: &str = "transaction";
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct evm_t {}

pub fn to_evm<'a>(ptr: *mut evm_t) -> Option<&'a mut Evm<'a, EthereumWiring<GoStorage<'a>, ()>>> {
    if ptr.is_null() {
        None
    } else {
        let evm = unsafe { &mut *(ptr as *mut Evm<'a, EthereumWiring<GoStorage<'a>, ()>>) };
        Some(evm)
    }
}

// initialize vm instance with handler
#[no_mangle]
pub extern "C" fn init_vm(// pre_execution: Option<&PreExecutionHandler>,
    // post_execution: Option<&PostExecutionHandler>
) -> *mut evm_t {
    let context = Context::default();
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
pub extern "C" fn execute_evm(
    vm_ptr: *mut evm_t,
    db: Db,               // -> Block Cache State from KVStore
    block: ByteSliceView, // -> block JSON Data
    tx: ByteSliceView,    // -> tx JSON Data
) -> UnmanagedVector {
    let evm: &mut Evm<'_, EthereumWiring<GoStorage<'_>, ()>> = match to_evm(vm_ptr) {
        Some(vm) => vm,
        None => {
            panic!("Failed to get VM");
        }
    };
    let block: BlockEnv = serde_json::from_str(
        &String::from_utf8(
            block
                .read()
                .unwrap()
                //.ok_or_else(|| Error::unset_arg(BLOCK))?
                .to_vec(),
        )
        .unwrap(),
    )
    .unwrap();

    let tx: TxEnv = serde_json::from_str(
        &String::from_utf8(
            tx.read()
                .unwrap()
                //.ok_or_else(|| Error::unset_arg(TRANSACTION))?
                .to_vec(),
        )
        .unwrap(),
    )
    .unwrap();

    let db = GoStorage::new(&db);
    evm.context = Context::new_with_db(db);
    evm.context.evm.inner.env.block = block;
    evm.context.evm.inner.env.tx = tx;

    let result = evm.transact_commit();

    match result {
        Ok(res) => {
            return UnmanagedVector::from_data(&res);
        }
        Err(e) => panic!("EVM Execution Result Error: {}", e),
    }
}
