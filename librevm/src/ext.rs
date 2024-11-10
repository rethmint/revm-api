use std::sync::Arc;

use alloy_primitives::B256;
use bytes::Buf;
use revm::{handler::register::EvmHandler, Database};
use revmc::{eyre::Result, EvmCompilerFn};

use crate::jit::{LevelDB, LEVELDB_PATH};

pub struct ExternalContext {}

revmc::extern_revmc! {
    fn fibonacci;
}

impl ExternalContext {
    pub fn new() -> Self {
        Self {}
    }

    fn get_function(&self, bytecode_hash: B256) -> Option<EvmCompilerFn> {
        // TODO: Restrain from initializing db every get function call
        let leveldb = LevelDB::init();
        let mut key = bytecode_hash.as_slice().get_i32();

        println!("checking bytecode hash {:#?}", bytecode_hash);

        // [ count key ]
        // 0x000000000000001
        // [ fn key ]
        // 0x000001000000001
        // 10th bit from the right is set
        key |= 1 << 9;

        let maybe_f = leveldb.get(key).unwrap_or(None);
        if let Some(f) = maybe_f {
            let fn_name = String::from_utf8(f).unwrap();

            let lib;
            let f = {
                lib = unsafe { libloading::Library::new(LEVELDB_PATH) }
                    .expect("Should've loaded linked library");
                let f: libloading::Symbol<'_, revmc::EvmCompilerFn> =
                    unsafe { lib.get(fn_name.as_bytes()).expect("Should've got library") };
                *f
            };

            return Some(f);
        }

        None
    }

    fn inc_hash_count(&self, bytecode_hash: B256) -> Result<()> {
        // TODO: Restrain from initializing db every inc call
        let leveldb = LevelDB::init();
        let key = bytecode_hash.as_slice().get_i32();

        let count = leveldb.get(key).unwrap_or(None);
        let new_count = count.as_ref().map_or(1, |v| {
            let bytes: [u8; 4] = v.as_slice().try_into().unwrap_or([0, 0, 0, 0]);
            i32::from_be_bytes(bytes) + 1
        });

        leveldb.put(key, &new_count.to_be_bytes(), false).unwrap();
        Ok(())
    }
}

// This `+ 'static` bound is only necessary here because of an internal cfg feature.
pub fn register_handler<DB: Database>(handler: &mut EvmHandler<'_, ExternalContext, DB>) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let bytecode_hash = interpreter.contract.hash.unwrap_or_default();
        context
            .external
            .inc_hash_count(bytecode_hash)
            .expect("increment failed");

        if let Some(f) = context.external.get_function(bytecode_hash) {
            Ok(unsafe { f.call_with_interpreter_and_memory(interpreter, memory, context) })
        } else {
            prev(frame, memory, tables, context)
        }
    });
}
