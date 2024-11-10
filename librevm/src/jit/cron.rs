use alloy_primitives::B256;
use revmc::eyre::{Context, Result};
use tokio::task::JoinHandle;

use crate::jit::{JitCfg, JitUnit, RuntimeJit};

use super::LevelDB;

pub struct Cronner {
    // unix
    interval: u64,
    database: LevelDB<'static, i32>,
}

impl Cronner {
    pub fn new_with_db(interval: u64, database: LevelDB<'static, i32>) -> Self {
        Self { interval, database }
    }

    pub fn start_routine(&self) -> JoinHandle<()> {
        let leveldb = self.database.clone();
        tokio::spawn(async move {
            let cron_future = Cronner::cron(leveldb);
            let _ = tokio::join!(cron_future);
        })
    }

    pub async fn cron(leveldb: LevelDB<'static, i32>) {}

    pub fn jit(&self, bytecode: &[u8], bytecode_hash: B256) -> Result<()> {
        println!("Setting function {:#?}", bytecode_hash);
        let unit = JitUnit::new("Fibonacci", bytecode.to_vec(), 70);
        let runtime_jit = RuntimeJit::new(unit, JitCfg::default());
        runtime_jit.compile().wrap_err("Compilation fail")
    }
}
