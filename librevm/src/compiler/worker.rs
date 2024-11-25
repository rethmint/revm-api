use std::sync::{Arc, RwLock};

use alloy_primitives::B256;

use super::{
    aot::{AotCfg, RuntimeAot},
    SledDB, SledDBKeySlice,
};
use crate::{
    compiler::{KeyPrefix, SledDbKey},
    runtime::get_runtime,
    utils::ivec_to_u64,
};

pub struct CompileWorker {
    pub threshold: u64,
    sled_db: Arc<RwLock<SledDB<SledDBKeySlice>>>,
    aot_runtime: Arc<RuntimeAot>,
}

impl CompileWorker {
    pub fn new(threshold: u64, sled_db: Arc<RwLock<SledDB<SledDBKeySlice>>>) -> Self {
        Self {
            threshold,
            sled_db,
            aot_runtime: Arc::new(RuntimeAot::new(AotCfg::default())),
        }
    }

    pub fn work(&mut self, code_hash: B256, bytecode: revm::primitives::Bytes) {
        let key = SledDbKey::with_prefix(code_hash, KeyPrefix::Count);
        let count = {
            let db_read = match self.sled_db.read() {
                Ok(lock) => lock,
                Err(poisoned) => poisoned.into_inner(),
            };
            let count_bytes = db_read.get(*key.as_inner()).unwrap_or(None);
            count_bytes.and_then(|v| ivec_to_u64(&v)).unwrap_or(0)
        };
        // 1. read bytecodeahash count from db
        let new_count = count + 1;

        let sled_db = Arc::clone(&self.sled_db);
        let aot_runtime = self.aot_runtime.clone();
        let threshold = self.threshold;

        let runtime = get_runtime();
        runtime.spawn(async move {
            // 2. check if bytecode is all zeros
            if code_hash.iter().all(|&b| b == 0) {
                return;
            }
            // 3. check condition of compile
            if new_count == threshold {
                // Compile the bytecode
                let label = SledDbKey::with_prefix(code_hash, KeyPrefix::SOPath)
                    .to_b256()
                    .to_string()
                    .leak();
                match aot_runtime.compile(label, bytecode.as_ref()) {
                    Ok(so_path) => {
                        let db_write = match sled_db.write() {
                            Ok(lock) => lock,
                            Err(poisoned) => poisoned.into_inner(),
                        };
                        db_write
                            .put(
                                *SledDbKey::with_prefix(code_hash, KeyPrefix::SOPath).as_inner(),
                                so_path.to_str().unwrap().as_bytes(),
                            )
                            .unwrap();
                    }
                    Err(err) => {
                        eprintln!("Failed to JIT compile: {:?}", err.to_string());
                        return;
                    }
                }
            }
            // 4. new count db commit
            {
                let db_write = match sled_db.write() {
                    Ok(lock) => lock,
                    Err(poisoned) => poisoned.into_inner(),
                };
                db_write
                    .put(*key.as_inner(), &new_count.to_be_bytes())
                    .unwrap();
            }
        });
    }
}
