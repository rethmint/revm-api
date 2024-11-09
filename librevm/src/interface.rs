use revm::{primitives::SpecId, Context, Evm, EvmBuilder};

use crate::{
    db::Db,
    error::set_error,
    ext::{register_handler, ExternalContext},
    gstorage::GoStorage,
    memory::{ByteSliceView, UnmanagedVector},
    utils::{build_flat_buffer, set_evm_env},
};

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
#[no_mangle]
pub extern "C" fn init_vm(default_spec_id: u8) -> *mut evm_t {
    let db = Db::default();
    let go_storage = GoStorage::new(&db);
    let spec = SpecId::try_from_u8(default_spec_id).unwrap_or(SpecId::CANCUN);

    let builder = EvmBuilder::default();
    let evm = builder
        .with_db(go_storage)
        .with_spec_id(spec)
        .with_external_context(ExternalContext::new())
        .append_handler_register(register_handler)
        .build();

    let vm = Box::into_raw(Box::new(evm));

    vm as *mut evm_t
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
