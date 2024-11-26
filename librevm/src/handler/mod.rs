mod bank_end_handler;
mod account_end_handler;

use std::sync::Arc;

use bank_end_handler::*;
use account_end_handler::account_end_handler;
use revm::{ handler::register::EvmHandler, primitives::ResultAndState, Database };

use crate::ext::ExternalContext;

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
                | revm::primitives::Bytecode::LegacyAnalyzed(_) =>
                    context.external.work(code_hash, bytecode.original_bytes()),
                // eof and eip7702 not supoorted by revmc llvm
                revm::primitives::Bytecode::Eip7702(_) => {}
                revm::primitives::Bytecode::Eof(_) => {}
            }

            prev(frame, memory, tables, context)
        }
    });
    let prev_end = handler.post_execution.end.clone();
    // summarize erc20 contract event
    handler.post_execution.end = Arc::new(move |context, execution_results| {
        let result: ResultAndState = execution_results.unwrap();
        // record new account address created
        account_end_handler(result.state);
        // record erc20 events with result result
        // result.result.logs().

        prev_end(context, execution_results)
    });
}
