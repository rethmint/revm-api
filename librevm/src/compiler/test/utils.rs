use std::sync::{Arc, RwLock};

use crate::{
    compiler::{CompileWorker, SledDB},
    utils::ivec_to_u64,
};
use alloy_primitives::{hex, Bytes, B256};
use revm::primitives::{Bytecode, SpecId};

type Db = Arc<RwLock<SledDB<B256>>>;

// Helper
pub fn init_sled_db() -> Db {
    Arc::new(RwLock::new(SledDB::<B256>::init()))
}

pub fn init_worker(threshold: u64) -> (Db, CompileWorker) {
    let db = init_sled_db();
    (db.clone(), CompileWorker::new(threshold, db))
}

pub fn init_fib_bytecode() -> Bytecode {
    let fib_bin = "0x6080604052348015600e575f5ffd5b5061010c8061001c5f395ff3fe6080604052348015600e575f5ffd5b50600436106026575f3560e01c806361047ff414602a575b5f5ffd5b60396035366004608c565b604b565b60405190815260200160405180910390f35b5f815f03605957505f919050565b81600103606857506001919050565b6073603560028460b6565b607e603560018560b6565b6086919060c6565b92915050565b5f60208284031215609b575f5ffd5b5035919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115608657608660a2565b80820180821115608657608660a256fea264697066735822122075f2b7835a429cbebf20df7d12b06806472a0714419b9b29eac058b3a7f6d80c64736f6c634300081b0033";
    let bytes_slice_res = hex::decode(fib_bin);
    assert!(
        bytes_slice_res.is_ok(),
        "Unexpected error: {:?}",
        bytes_slice_res.unwrap_err()
    );

    let bytes_slice = bytes_slice_res.unwrap();
    let bytes = Bytes::from(bytes_slice);

    Bytecode::new_legacy(bytes.clone())
}

pub fn initiate_compiler_work() -> (Db, B256) {
    let bytecode = init_fib_bytecode();
    let code_hash = bytecode.hash_slow();
    let spec_id = SpecId::OSAKA;

    let threshold = 1_000;
    let (db, mut worker) = init_worker(threshold);

    worker.work(spec_id, code_hash, bytecode.bytes());

    (db, code_hash)
}

pub fn count_reference(db: Db, code_hash: B256) -> u64 {
    let db_read = match db.read() {
        Ok(lock) => lock,
        Err(poisoned) => poisoned.into_inner(),
    };
    let count_bytes = db_read.get(code_hash).unwrap_or(None);
    count_bytes.and_then(|v| ivec_to_u64(&v)).unwrap_or(0)
}
