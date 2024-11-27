use alloy_primitives::B256;
use revm::{handler::register::EvmHandler, Database};
use revmc::EvmCompilerFn;
use std::env;
use std::{
    panic::{self, AssertUnwindSafe},
    path::PathBuf,
    sync::Arc,
};

use crate::{compiler::CompileWorker, error::ExtError};

pub struct ExternalContext {
    compile_worker: &'static mut CompileWorker,
}

#[inline]
fn so_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".rethmint").join("output")
}

impl ExternalContext {
    pub fn new(compile_worker: &'static mut CompileWorker) -> Self {
        Self { compile_worker }
    }

    fn get_function(
        &self,
        code_hash: B256,
    ) -> Result<Option<(EvmCompilerFn, libloading::Library)>, ExtError> {
        let label = code_hash.to_string();
        let so_file = so_path().join(label).join("a.so");

        //if let Ok(true) = so_file.try_exists() {
        //    let lib;
        //    let f = {
        //        lib = (unsafe { libloading::Library::new(&so_file) }).map_err(|err| {
        //            ExtError::LibLoadingError {
        //                err: err.to_string(),
        //            }
        //        })?;
        //        let f: libloading::Symbol<'_, revmc::EvmCompilerFn> = unsafe {
        //            lib.get(code_hash.to_string().as_ref()).map_err(|err| {
        //                ExtError::GetSymbolError {
        //                    err: err.to_string(),
        //                }
        //            })?
        //        };
        //        *f
        //    };
        //
        //    return Ok(Some((f, lib)));
        //}

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
