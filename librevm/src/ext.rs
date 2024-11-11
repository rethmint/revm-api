use std::{path::Path, sync::Arc};

use alloy_primitives::B256;
use revm::{handler::register::EvmHandler, Database};
use revmc::{eyre::Result, EvmCompilerFn};

use crate::{
    jit::{KeyPrefix, QueryKey, QueryKeySlice, SledDB, JIT_OUT_PATH, JIT_THRESHOLD},
    SLED_DB,
};

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
        let sled_db = SLED_DB.get_or_init(|| Arc::new(SledDB::<QueryKeySlice>::init()));
        let label_key = QueryKey::with_prefix(bytecode_hash, KeyPrefix::Label);

        let prefix_zeros = &bytecode_hash[0..10];
        if prefix_zeros.iter().all(|&byte| byte == 0) {
            // Skip processing if it starts with 10 zeros
            return None;
        }

        println!("Checking count for bytecode hash {:#?}", bytecode_hash);
        let maybe_label = sled_db.get(*label_key.as_inner()).unwrap_or(None);
        if let Some(label) = maybe_label {
            let fn_label = String::from_utf8(label.to_vec()).unwrap();

            let lib;
            let f = {
                let jit_out_path = Path::new(JIT_OUT_PATH);
                let so_path = jit_out_path.join(&fn_label).join("a.so");

                lib = unsafe { libloading::Library::new(so_path) }
                    .expect("Should've loaded linked library");
                let f: libloading::Symbol<'_, revmc::EvmCompilerFn> =
                    unsafe { lib.get(fn_label.as_bytes()).expect("Should've got library") };
                *f
            };

            return Some(f);
        }
        //
        None
    }

    fn update_bytecode_reference(&self, bytecode: &[u8], bytecode_hash: B256) -> Result<()> {
        // TODO: Restrain from initializing db every inc call
        let sled_db = SLED_DB.get_or_init(|| Arc::new(SledDB::<QueryKeySlice>::init()));
        let count_key = QueryKey::with_prefix(bytecode_hash, KeyPrefix::Count);

        let count = sled_db.get(*count_key.as_inner()).unwrap_or(None);
        let new_count = count.as_ref().map_or(1, |v| {
            let bytes: [u8; 4] = v.to_vec().as_slice().try_into().unwrap_or([0, 0, 0, 0]);
            i32::from_be_bytes(bytes) + 1
        });

        sled_db
            .put(*count_key.as_inner(), &new_count.to_be_bytes(), true)
            .unwrap();

        // 9 cause 10 can cause unexpected behavior
        if new_count > JIT_THRESHOLD - 1 {
            let label_key = QueryKey::with_prefix(bytecode_hash, KeyPrefix::Label);
            if let None = sled_db.get(*label_key.as_inner()).unwrap_or(None) {
                let bytecode_key = QueryKey::with_prefix(bytecode_hash, KeyPrefix::Bytecode);
                sled_db
                    .put(*bytecode_key.as_inner(), bytecode, true)
                    .unwrap();
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
        let bytecode_hash = interpreter.contract.hash.unwrap_or_default();
        let bytecode = interpreter.contract.bytecode.original_byte_slice();

        context
            .external
            .update_bytecode_reference(bytecode, bytecode_hash)
            .expect("Update failed");

        if let Some(f) = context.external.get_function(bytecode_hash) {
            println!("Calling extern function on hash: {bytecode_hash:#?}");
            Ok(unsafe { f.call_with_interpreter_and_memory(interpreter, memory, context) })
        } else {
            prev(frame, memory, tables, context)
        }
    });
}
