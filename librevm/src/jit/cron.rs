use alloy_primitives::B256;
use revmc::eyre::{Context, Result};
use tokio::task::JoinHandle;

use crate::jit::{JitCfg, JitUnit, RuntimeJit};

use super::LevelDB;

const JIT_THRESHOLD: i32 = 10;

pub struct Cronner {
    // ms
    interval: u64,
    database: LevelDB<'static, i32>,
}

impl Cronner {
    pub fn new_with_db(interval: u64, database: LevelDB<'static, i32>) -> Self {
        Self { interval, database }
    }

    pub fn start_routine(&self) -> JoinHandle<()> {
        let interval = self.interval.clone();
        let leveldb = self.database.clone();
        tokio::spawn(async move {
            let cron_future = Cronner::cron(interval, leveldb);
            let _ = tokio::join!(cron_future);
        })
    }

    pub async fn cron(interval: u64, leveldb: LevelDB<'static, i32>) {

        //let leveldb = LevelDB::init();
        //let key = bytecode_hash.as_slice().get_i32();
        //
        //let count_bytes = leveldb.get(key).unwrap_or(None);
        //let count = count_bytes.as_ref().map_or(1, |v| {
        //    let bytes: [u8; 4] = v.as_slice().try_into().unwrap_or([0, 0, 0, 0]);
        //    i32::from_be_bytes(bytes) + 1
        //});
        //
        //if count > JIT_THRESHOLD {
        //    //return Some(EvmCompilerFn::new(fibonacci));
        //    panic!();
        //}
        //
        //None
    }

    pub fn jit(&self, bytecode: &[u8], bytecode_hash: B256) -> Result<()> {
        println!("Jit in progress {:#?}", bytecode_hash);
        let unit = JitUnit::new("Fibonacci", bytecode.to_vec(), 70);
        let runtime_jit = RuntimeJit::new(unit, JitCfg::default());
        runtime_jit.compile().wrap_err("Compilation fail")
    }
}
