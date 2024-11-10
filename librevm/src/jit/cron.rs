use std::sync::Arc;

use alloy_primitives::B256;

use crate::jit::{JitCfg, JitUnit, RuntimeJit};

use super::LevelDB;

pub struct Cronner<'a> {
    // unix
    interval: u64,
    database: LevelDB<'a, i32>,
}

impl<'a> Cronner<'a> {
    pub fn new_with_db(interval: u64, database: LevelDB<'a, i32>) -> Self {
        Self { interval, database }
    }

    pub fn jit(&self, bytecode: &[u8], bytecode_hash: B256) {
        println!("Setting function {:#?}", bytecode_hash);
        let unit = JitUnit::new("Fibonacci", bytecode.to_vec(), 70);
        let runtime_jit = RuntimeJit::new(unit, JitCfg::default());
        runtime_jit.compile().unwrap()
    }
}
