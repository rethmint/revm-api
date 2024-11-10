use std::sync::Arc;

use alloy_primitives::{hex, B256};
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
