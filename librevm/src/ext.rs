use std::sync::Arc;

use alloy_primitives::{hex, B256};
use bytes::Buf;
use revm::{handler::register::EvmHandler, Database};
use revmc::EvmCompilerFn;

use crate::jit::LevelDB;

pub struct ExternalContext<'a> {
    database: LevelDB<'a, i32>,
}

revmc::extern_revmc! {
    fn fibonacci;
}

impl<'a> ExternalContext<'a> {
    pub fn new_with_db(database: LevelDB<'a, i32>) -> Self {
        Self { database }
    }

    fn get_function(&self, bytecode_hash: B256) -> Option<EvmCompilerFn> {
        if bytecode_hash == hex!("ab1ad1211002e1ddb8d9a4ef58a902224851f6a0273ee3e87276a8d21e649ce8")
        {
            //return Some(EvmCompilerFn::new(fibonacci));
            panic!();
        }

        None
    }

    fn inc_hash_count(&self, bytecode_hash: B256) -> Option<EvmCompilerFn> {
        let key = bytecode_hash.as_slice().get_i32();
        let current_count = self.database.get(key).unwrap_or(None);
        let count = current_count.map_or(1, |v| {
            let bytes: [u8; 4] = v.as_slice().try_into().unwrap_or([0, 0, 0, 0]);
            i32::from_be_bytes(bytes) + 1
        });

        self.database.put(123, &count.to_be_bytes(), false).unwrap();
        None
    }
}

// This `+ 'static` bound is only necessary here because of an internal cfg feature.
pub fn register_handler<DB: Database>(handler: &mut EvmHandler<'_, ExternalContext, DB>) {
    let prev = handler.execution.execute_frame.clone();
    handler.execution.execute_frame = Arc::new(move |frame, memory, tables, context| {
        let interpreter = frame.interpreter_mut();
        let bytecode_hash = interpreter.contract.hash.unwrap_or_default();

        if let Some(f) = context.external.get_function(bytecode_hash) {
            Ok(unsafe { f.call_with_interpreter_and_memory(interpreter, memory, context) })
        } else {
            prev(frame, memory, tables, context)
        }
    });
}
