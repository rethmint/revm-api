use std::{ collections::{ BTreeMap, HashMap }, convert::Infallible, str::FromStr };

use k256::ecdsa::SigningKey;
use revm::{
    db::{ EmptyDB, EmptyDBTyped },
    inspector_handle_register,
    inspectors::NoOpInspector,
    CacheState,
    Context,
    Evm,
    EvmBuilder,
    EvmContext,
    EvmHandler,
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

use crate::{ ByteSliceView, GoApi, UnmanagedVector };

// byte slice view: golang data type
// unamangedvector: ffi safe vector data type compliants with rust's ownership and data types, for returning optional error value

/**
 * idea sep 17
 * 1. Receive env from GoApi and initialize context
 * 2. Use the context to initialize vm
 * 3. Use the VM with the env to call 'call' and 'create'
 * */

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct vm_t {}

pub fn to_vm(ptr: *mut vm_t) -> Option<&'static mut Evm<'static, EthereumWiring<EmptyDB, ()>>> {
    if ptr.is_null() {
        None
    } else {
        let c = unsafe { &mut *(ptr as *mut Evm<'static, EthereumWiring<EmptyDB, ()>>) };
        Some(c)
    }
}
#[no_mangle]
pub extern "C" fn allocate_executor() -> *mut vm_t {
    let builder = EvmBuilder::default().with_default_db().with_default_ext_ctx();

    let mainnet_handler = EvmHandler::<'_, EthereumWiring<EmptyDB, ()>>::mainnet_with_spec(
        SpecId::CANCUN
    );
    let builder = builder.with_handler(mainnet_handler);

    let evm = builder.build();

    let executor = Box::into_raw(Box::new(evm));
    executor as *mut vm_t
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CallRequest {
    //pub pre: HashMap<Address, AccountInfo>,
    pub transaction: CallTransaction,
    pub out: Option<Bytes>,
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CallTransaction {
    pub data: Bytes,
    pub gas_limit: U256,
    pub gas_price: Option<U256>,
    pub nonce: U256,
    pub secret_key: B256,
    pub sender: Option<Address>,
    pub to: Option<Address>,
    pub value: U256,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,

    pub access_lists: Vec<Option<AccessList>>,
    pub authorization_list: Vec<Authorization>,
    pub blob_versioned_hashes: Vec<B256>,
    pub max_fee_per_blob_gas: Option<U256>,
}

pub fn recover_address(private_key: &[u8]) -> Option<Address> {
    let key = SigningKey::from_slice(private_key).ok()?;
    let public_key = key.verifying_key().to_encoded_point(false);
    Some(Address::from_raw_public_key(&public_key.as_bytes()[1..]))
}

#[no_mangle]
pub extern "C" fn initialize(
    //vm_ptr: *mut vm_t,
    //api: GoApi,
    call_request_bytes: ByteSliceView
    //errmsg: Option<&mut UnmanagedVector>,
) -> UnmanagedVector {
    let mut env = Box::<EnvWiring<EthereumWiring<&'static mut State<EmptyDB>, ()>>>::default();

    let call_request_str = Option::<String>::from(call_request_bytes).unwrap();
    let call_request: CallRequest = serde_json::from_str(call_request_str.as_ref()).unwrap();

    // change tx env
    env.tx.caller = if let Some(address) = call_request.transaction.sender {
        address
    } else {
        recover_address(call_request.transaction.secret_key.as_slice()).unwrap()
    };
    env.tx.gas_price = call_request.transaction.gas_price
        .or(call_request.transaction.max_fee_per_gas)
        .unwrap_or_default();
    env.tx.gas_priority_fee = call_request.transaction.max_priority_fee_per_gas;
    // EIP-4844
    env.tx.blob_hashes = call_request.transaction.blob_versioned_hashes;
    env.tx.max_fee_per_blob_gas = call_request.transaction.max_fee_per_blob_gas;

    // TODO: add saturating to
    //env.tx.gas_limit = call_request.transaction.gas_limit;
    env.tx.gas_limit = 10;

    env.tx.data = call_request.transaction.data.clone();

    env.tx.nonce = u64::try_from(call_request.transaction.nonce).unwrap();
    env.tx.value = call_request.transaction.value;

    //env.tx.access_list = call_request
    //    .transaction
    //    .access_lists
    //    .get(test.indexes.data)
    //    .and_then(Option::as_deref)
    //    .cloned()
    //    .unwrap_or_default();

    //env.tx.authorization_list = auth_list;

    let to = match call_request.transaction.to {
        Some(add) => TxKind::Call(add),
        None => TxKind::Create,
    };
    env.tx.transact_to = to;

    let mut state = revm::db::State::builder().build();

    let mut evm: Evm<'_, EthereumWiring<&mut State<EmptyDBTyped<Infallible>>, ()>> = Evm::<
        EthereumWiring<&mut State<EmptyDB>, ()>
    >
        ::builder()
        .with_db(&mut state)
        .with_default_ext_ctx()
        .modify_env(|e| e.clone_from(&env))
        .with_spec_id(CANCUN)
        .build();

    let res = evm.transact_commit();
    println!("Result, {:#?}", res);

    UnmanagedVector::new(None)
}
//
pub fn execute_evm(vm_ptr: *mut vm_t, env: Env<BlockEnv, TxEnv>) {
    match env.tx.transact_to {
        TxKind::Call(Address) => evm_call(vm_ptr, env),
        TxKind::Create => evm_create(vm_ptr, env),
    }
}
fn evm_call(vm_ptr: *mut vm_t, env: Env<BlockEnv, TxEnv>) -> UnmanagedVector {
    let env = Env::boxed(env.cfg, env.block, env.tx);

    let mut cache = cache_state.clone();
    cache.set_state_clear_flag(SpecId::enabled(SpecId::CANCUN, SpecId::SPURIOUS_DRAGON));
    let mut state = revm::db::State
        ::builder()
        .with_cached_prestate(cache)
        .with_bundle_update()
        .build();

    let mut evm = Evm::<EthereumWiring<&mut State<EmptyDB>, ()>>
        ::builder()
        .with_db(&mut state)
        .with_default_ext_ctx()
        .modify_env(|e| e.clone_from(&env))
        .with_spec_id(CANCUN)
        .build();

    let res = evm.transact_commit();
    println!("Result, {:#?}", res);

    UnmanagedVector::new(None)
}
fn evm_create(vm_ptr: *mut vm_t, env: Env<BlockEnv, TxEnv>) -> UnmanagedVector {
    let env = Env::boxed(env.cfg, env.block, env.tx);

    let mut cache = cache_state.clone();
    cache.set_state_clear_flag(SpecId::enabled(SpecId::CANCUN, SpecId::SPURIOUS_DRAGON));
    let mut state = revm::db::State
        ::builder()
        .with_cached_prestate(cache)
        .with_bundle_update()
        .build();

    let mut evm = Evm::<EthereumWiring<&mut State<EmptyDB>, ()>>
        ::builder()
        .with_db(&mut state)
        .with_default_ext_ctx()
        .modify_env(|e| e.clone_from(&env))
        .with_spec_id(CANCUN)
        .build();

    let res = evm.transact_commit();
    println!("Result, {:#?}", res);

    UnmanagedVector::new(None)
}
