use std::sync::Mutex;

use once_cell::sync::Lazy;
use revm::{primitives::SpecId, Context, Evm, EvmBuilder};
use tokio::task::JoinHandle;

use crate::{
    db::Db,
    error::set_error,
    ext::{register_handler, ExternalContext},
    gstorage::GoStorage,
    jit::{Cronner, LevelDB},
    memory::{ByteSliceView, UnmanagedVector},
    utils::{build_flat_buffer, set_evm_env},
};

static CRON_HANDLE: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

// byte slice view: golang data type
// unamangedvector: ffi safe vector data type compliants with rust's ownership and data types, for returning optional error value
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
#[tokio::main]
#[no_mangle]
pub async extern "C" fn init_vm(default_spec_id: u8) -> *mut evm_t {
    let db = Db::default();
    let go_storage = GoStorage::new(&db);
    let spec = SpecId::try_from_u8(default_spec_id).unwrap_or(SpecId::CANCUN);

    let leveldb = LevelDB::init();
    initiate_cron_job(leveldb);

    let ext = ExternalContext::new();
    let builder = EvmBuilder::default();
    let evm = builder
        .with_db(go_storage)
        .with_spec_id(spec)
        .with_external_context(ext)
        .append_handler_register(register_handler)
        .build();

    let vm = Box::into_raw(Box::new(evm));

    vm as *mut evm_t
}

fn initiate_cron_job(leveldb: LevelDB<'static, i32>) {
    let interval_ms = 10_000;
    let cronner = Cronner::new_with_db(interval_ms, leveldb.clone());
    let cron_handle = cronner.start_routine();
    *CRON_HANDLE.lock().unwrap() = Some(cron_handle);
}

#[no_mangle]
pub extern "C" fn release_vm(vm: *mut evm_t) {
    if !vm.is_null() {
        // this will free cache when it goes out of scope
        let _ = unsafe { Box::from_raw(vm as *mut Evm<(), GoStorage>) };
    }
}

// VM initializer
#[no_mangle]
pub extern "C" fn execute_tx(
    vm_ptr: *mut evm_t,
    db: Db,
    block: ByteSliceView,
    tx: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>,
) -> UnmanagedVector {
    let evm = match to_evm(vm_ptr) {
        Some(vm) => vm,
        None => {
            panic!("Failed to get VM");
        }
    };
    let go_storage = GoStorage::new(&db);

    evm.context.evm.db = go_storage;

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
pub extern "C" fn query_tx(
    vm_ptr: *mut evm_t,
    db: Db,
    block: ByteSliceView,
    tx: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>,
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
