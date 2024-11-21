use alloy_primitives::B256;
use revm::{ handler::register::EvmHandler, Database };
use revmc::{ eyre::Result, EvmCompilerFn };
use tokio::sync::Mutex;
use std::sync::{ Arc, RwLock };

use crate::{
    aot::{ CompilationQueue, KeyPrefix, SledDB, SledDBKeySlice, SledDbKey },
    runtime::get_runtime,
    utils::{ ivec_to_pathbuf, ivec_to_u64 },
    SLED_DB,
};

#[derive(Clone)]
pub struct ExternalContext {
    compilation_queue: Arc<Mutex<CompilationQueue>>,
}

impl ExternalContext {
    pub fn new(compilation_queue: Arc<Mutex<CompilationQueue>>) -> Self {
        Self { compilation_queue }
    }

    fn get_function(
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

    async fn update_bytecode_reference(
        &mut self,
        code_hash: Arc<B256>,
        bytecode: Arc<revm::primitives::Bytes>
    ) -> Result<()> {
        let code_hash = (*code_hash).clone();
        let bytecode = (*bytecode).clone();
        let sled_db = SLED_DB.get_or_init(||
            Arc::new(RwLock::new(SledDB::<SledDBKeySlice>::init()))
        );
        let key = SledDbKey::with_prefix(code_hash, KeyPrefix::Count);

        let count = {
            let db_read = match sled_db.read() {
                Ok(lock) => lock,
                Err(poisoned) => poisoned.into_inner(),
            };
            let count_bytes = db_read.get(*key.as_inner()).unwrap_or(None);
            count_bytes.and_then(|v| ivec_to_u64(&v)).unwrap_or(0)
        };

        let new_count = count + 1;
        {
            let db_write = match sled_db.write() {
                Ok(lock) => lock,
                Err(poisoned) => poisoned.into_inner(),
            };
            db_write.put(*key.as_inner(), &new_count.to_be_bytes()).unwrap();
        }

        // if new count equals the threshold, push to queue
        if let Ok(queue) = self.compilation_queue.try_lock() {
            if new_count == queue.threshold {
                queue.push(code_hash.clone(), bytecode.clone()).await;
            }
        }

        Ok(())
    }
}

// This `+ 'static` bound is only necessary here because of an internal cfg feature.
pub fn register_handler<DB: Database>(handler: &mut EvmHandler<'_, ExternalContext, DB>) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let code_hash = interpreter.contract.hash.unwrap_or_default();
        if let Some((f, _lib)) = context.external.get_function(code_hash).unwrap() {
            println!("Executing with AOT Compiled Fn\n");
            Ok(unsafe { f.call_with_interpreter_and_memory(interpreter, memory, context) })
        } else {
            // if there are no function in aot compiled lib, count the bytecode reference
            let bytecode = context.evm.db.code_by_hash(code_hash).unwrap_or_default();
            match bytecode {
                | revm::primitives::Bytecode::LegacyRaw(_)
                | revm::primitives::Bytecode::LegacyAnalyzed(_) => {
                    let code_hash = Arc::new(code_hash.clone());
                    let bytecode = Arc::new(bytecode.original_bytes().clone());
                    let external = Arc::new(Mutex::new(context.external.clone()));
                    let runtime = get_runtime();
                    runtime.spawn(async move {
                        if let Ok(mut external) = external.try_lock() {
                            external
                                .update_bytecode_reference(code_hash, bytecode).await
                                .unwrap_or_else(|err|
                                    eprintln!(
                                        "Update Bytecode Reference Failed: {:?}",
                                        err.to_string()
                                    )
                                );
                        }
                    });
                }
                // eof and eip7702 not supoorted by revmc llvm
                _ => {}
            }

            prev(frame, memory, tables, context)
        }
    });
}
