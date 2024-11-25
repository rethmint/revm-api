use alloy_primitives::B256;
use revmc::{ eyre::Result, EvmCompilerFn };
use std::sync::{ Arc, RwLock };

use crate::{
    compiler::{ CompileWorker, KeyPrefix, SledDB, SledDBKeySlice, SledDbKey },
    utils::ivec_to_pathbuf,
    SLED_DB,
};

pub struct ExternalContext {
    compile_worker: &'static mut CompileWorker,
}

impl ExternalContext {
    pub fn new(compile_worker: &'static mut CompileWorker) -> Self {
        Self { compile_worker }
    }

    pub fn get_function(
        &self,
        code_hash: B256
    ) -> Result<Option<(EvmCompilerFn, libloading::Library)>> {
        let sled_db = SLED_DB.get_or_init(||
            Arc::new(RwLock::new(SledDB::<SledDBKeySlice>::init()))
        );
        let key = SledDbKey::with_prefix(code_hash, KeyPrefix::SOPath);

        let maybe_so_path = {
            let db_read = sled_db.read().expect("Failed to acquire read lock");
            db_read.get(*key.as_inner()).unwrap_or(None)
        };

        if let Some(so_path) = maybe_so_path {
            let so_path = ivec_to_pathbuf(&so_path).unwrap();
            let lib;
            let f = {
                lib = (unsafe { libloading::Library::new(&so_path) }).unwrap();
                let f: libloading::Symbol<'_, revmc::EvmCompilerFn> = unsafe {
                    lib.get(code_hash.to_string().as_ref()).unwrap()
                };
                *f
            };

            return Ok(Some((f, lib)));
        }

        Ok(None)
    }

    pub fn work(&mut self, code_hash: B256, bytecode: revm::primitives::Bytes) {
        self.compile_worker.work(code_hash, bytecode);
    }
}
