use std::sync::Arc;

use alloy_primitives::B256;
use revm::{handler::register::EvmHandler, Database};
use revmc::EvmCompilerFn;

pub struct ExternalContext;

impl ExternalContext {
    fn new() -> Self {
        Self
    }

    fn get_function(&self, bytecode_hash: B256) -> Option<EvmCompilerFn> {
        // Can use any mapping between bytecode hash and function.
        //if bytecode_hash == FIBONACCI_HASH {
        //    return Some(EvmCompilerFn::new(fibonacci));
        //}

        None
    }
}

// This `+ 'static` bound is only necessary here because of an internal cfg feature.
fn register_handler<DB: Database + 'static>(handler: &mut EvmHandler<'_, ExternalContext, DB>) {
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
