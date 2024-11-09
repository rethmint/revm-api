use alloy_primitives::B256;

use crate::jit::{JitCfg, JitUnit, RuntimeJit};

pub struct Cronner {}
impl Cronner {
    fn set_function(&self, bytecode: &[u8], bytecode_hash: B256) {
        println!("Setting function {:#?}", bytecode_hash);
        let unit = JitUnit::new("Fibonacci", bytecode.to_vec(), 70);
        let runtime_jit = RuntimeJit::new(unit, JitCfg::default());
        runtime_jit.compile().unwrap()
    }
}
