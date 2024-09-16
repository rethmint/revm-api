use crate::{error::RustError as Error, GoStorage};
use revm_primitives::Address;
use vm::Revm;

use crate::Db;

pub(crate) fn initialize_vm(
    vm: &mut Revm,
    db_handle: Db,
    allowed_publishers: Vec<Address>,
) -> Result<Vec<u8>, Error> {
    let mut storage = GoStorage::new(&db_handle);

    let output = vm.initialize(allowed_publishers)?;

    // write state change to storage
    push_write_set(&mut storage, output.write_set())?;

    let res = generate_result(output)?;
    to_vec(&res)
}

pub(crate) fn execute_contract(
    vm: &mut InitiaVM,
    gas_meter: &mut InitiaGasMeter,
    db_handle: Db,
    api: GoApi,
    env: Env,
    message: Message,
) -> Result<Vec<u8>, Error> {
    let mut storage = GoStorage::new(&db_handle);
    let mut table_storage = GoTableStorage::new(&db_handle);

    let state_view_impl =
        StateViewImpl::new_with_deserialize_config(&storage, vm.deserialize_config().clone());

    let output = vm.execute_message(
        gas_meter,
        &api,
        &env,
        &state_view_impl,
        &mut table_storage,
        message,
    )?;

    // push write set to storage
    push_write_set(&mut storage, output.write_set())?;

    let res = generate_result(output)?;
    to_vec(&res)
}

pub(crate) fn execute_script(
    vm: &mut InitiaVM,
    gas_meter: &mut InitiaGasMeter,
    db_handle: Db,
    api: GoApi,
    env: Env,
    message: Message,
) -> Result<Vec<u8>, Error> {
    let mut storage = GoStorage::new(&db_handle);
    let mut table_storage = GoTableStorage::new(&db_handle);

    // NOTE - storage passed as mut for iterator implementation
    let state_view_impl =
        StateViewImpl::new_with_deserialize_config(&storage, vm.deserialize_config().clone());

    let output = vm.execute_message(
        gas_meter,
        &api,
        &env,
        &state_view_impl,
        &mut table_storage,
        message,
    )?;

    // push write set to storage
    push_write_set(&mut storage, output.write_set())?;

    let res = generate_result(output)?;
    to_vec(&res)
}

// execute view function
pub(crate) fn execute_view_function(
    vm: &mut InitiaVM,
    gas_meter: &mut InitiaGasMeter,
    db_handle: Db,
    api: GoApi,
    env: Env,
    view_fn: ViewFunction,
) -> Result<Vec<u8>, Error> {
    let storage = GoStorage::new(&db_handle);
    let mut table_storage = GoTableStorage::new(&db_handle);

    let state_view_impl =
        StateViewImpl::new_with_deserialize_config(&storage, vm.deserialize_config().clone());

    let output = vm.execute_view_function(
        gas_meter,
        &api,
        &env,
        &state_view_impl,
        &mut table_storage,
        &view_fn,
    )?;

    to_vec(&output)
}

/////////////////////////////////////////
/// Storage Operation ///////////////////
/////////////////////////////////////////

fn write_op(
    go_storage: &mut GoStorage,
    ap: &AccessPath,
    blob_opt: &Op<Vec<u8>>,
) -> Result<(), BackendError> {
    let key = ap
        .to_bytes()
        .map_err(|_| BackendError::unknown("failed to encode access path"))?;
    match blob_opt {
        Op::New(blob) | Op::Modify(blob) => go_storage.set(&key, blob),
        Op::Delete => go_storage.remove(&key),
    }
}

pub fn push_write_set(
    go_storage: &mut GoStorage,
    write_set: &WriteSet,
) -> Result<(), BackendError> {
    for (ap, blob_opt) in write_set {
        write_op(go_storage, ap, blob_opt)?;
    }

    Ok(())
}
