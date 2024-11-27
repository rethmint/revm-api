use alloy_primitives::B256;
use revm::{handler::register::EvmHandler, Database};
use revmc::{
    eyre::{eyre, Result},
    EvmCompilerFn,
};
use std::{
    panic::{self, AssertUnwindSafe},
    sync::{Arc, RwLock},
};

use crate::{
    compiler::{CompileWorker, KeyPrefix, SledDB, SledDBKeySlice, SledDbKey},
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

    fn get_function(
        &self,
        code_hash: B256,
    ) -> Result<Option<(EvmCompilerFn, libloading::Library)>> {
        let sled_db =
            SLED_DB.get_or_init(|| Arc::new(RwLock::new(SledDB::<SledDBKeySlice>::init())));
        let key = SledDbKey::with_prefix(code_hash, KeyPrefix::SOPath);

        let maybe_so_path = {
            let db_read = sled_db.read().map_err(|err| eyre!(err.to_string()))?;
            db_read.get(*key.as_inner()).unwrap_or(None)
        };

        if let Some(so_path) = maybe_so_path {
            let so_path =
                ivec_to_pathbuf(&so_path).ok_or_else(|| eyre!("IVec to pathbuf error"))?;
            let lib;
            let f = {
                lib = (unsafe { libloading::Library::new(&so_path) })?;
                let f: libloading::Symbol<'_, revmc::EvmCompilerFn> =
                    unsafe { lib.get(code_hash.to_string().as_ref())? };
                *f
            };

            return Ok(Some((f, lib)));
        }

        Ok(None)
    }

    fn work(&mut self, code_hash: B256, bytecode: revm::primitives::Bytes) {
        self.compile_worker.work(code_hash, bytecode);
    }
}

// This `+ 'static` bound is only necessary here because of an internal cfg feature.
pub fn register_handler<DB: Database>(handler: &mut EvmHandler<'_, ExternalContext, DB>) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let code_hash = interpreter.contract.hash.unwrap_or_default();

        match context.external.get_function(code_hash) {
            Ok(None) => {
                // if there are no function in aot compiled lib, count the bytecode reference
                let bytecode = context.evm.db.code_by_hash(code_hash).unwrap_or_default();
                match bytecode {
                    revm::primitives::Bytecode::LegacyRaw(_)
                    | revm::primitives::Bytecode::LegacyAnalyzed(_) => {
                        context.external.work(code_hash, bytecode.original_bytes())
                    }
                    // eof and eip7702 not supoorted by revmc llvm
                    revm::primitives::Bytecode::Eip7702(_) => {}
                    revm::primitives::Bytecode::Eof(_) => {}
                }
                prev(frame, memory, tables, context)
            }
            Ok(Some((f, _lib))) => {
                println!("Executing with AOT Compiled Fn\n");
                let res = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
                    f.call_with_interpreter_and_memory(interpreter, memory, context)
                }));

                if let Err(err) = &res {
                    tracing::error!(
                        "Extern Fn Call: with bytecode hash {} {:#?}",
                        code_hash,
                        err
                    );
                }

                Ok(res.unwrap())
            }
            Err(err) => {
                tracing::error!("Get function: with bytecode hash {} {:#?}", code_hash, err);
                prev(frame, memory, tables, context)
            }
        }
    });
}
