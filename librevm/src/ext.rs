use std::{env, fs::File, io::Write, sync::Arc};

use alloy_primitives::B256;
use libloading::{Library, Symbol};
use revm::{handler::register::EvmHandler, Database};
use revmc::{eyre::Result, EvmCompilerFn};

use crate::{
    jit::{KeyPrefix, QueryKey, QueryKeySlice, SledDB},
    SLED_DB,
};

revmc::extern_revmc! {
    fn afn;
}

pub struct ExternalContext {}

impl ExternalContext {
    pub fn new() -> Self {
        Self {}
    }

    fn get_function(&self, bytecode_hash: B256) -> Option<EvmCompilerFn> {
        let sled_db = SLED_DB.get_or_init(|| Arc::new(SledDB::<QueryKeySlice>::init()));
        let so_key = QueryKey::with_prefix(bytecode_hash, KeyPrefix::SO);

        if let Some(so_bytes) = sled_db.get(*so_key.as_inner()).unwrap_or(None) {
            let temp_dir = env::temp_dir();
            let temp_file_path = temp_dir.join("a.so");

            if let Ok(mut file) = File::create(&temp_file_path) {
                file.write_all(&so_bytes).unwrap();

                let lib = unsafe { Library::new(&temp_file_path) }.unwrap();
                let f: Symbol<'_, EvmCompilerFn> = unsafe { lib.get("afn".as_bytes()).unwrap() };

                return Some(*f);
            } else {
                eprintln!("Failed to create temporary file");
            }
        }
        None
    }

    fn update_bytecode_reference(&self, bytecode_hash: B256) -> Result<()> {
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

        println!("Printing bytecode hash: {:#?}", &bytecode_hash);
        println!("Printing bytecode: {:#?}", &bytecode[0..10]);

        context
            .external
            .update_bytecode_reference(bytecode_hash)
            .expect("Update failed");

        if let Some(f) = context.external.get_function(bytecode_hash) {
            println!("Calling extern function on hash: {bytecode_hash:#?}");
            Ok(unsafe { f.call_with_interpreter_and_memory(interpreter, memory, context) })
        } else {
            prev(frame, memory, tables, context)
        }
    });
}
