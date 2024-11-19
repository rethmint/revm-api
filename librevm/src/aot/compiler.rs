use std::{ collections::VecDeque, path::PathBuf, sync::{ Arc, RwLock }, time };

use alloy_primitives::FixedBytes;
use revm::{ primitives::Bytecode, Database };
use revmc::eyre::Result;
use tokio::time::{ interval_at, Instant };

use super::{ QueryKeySlice, SledDB };
use crate::{
    aot::{ AotCfg, KeyPrefix, RuntimeAot },
    gstorage::GoStorage,
    storeutils::{ CodeHash, EvmStoreKey },
    utils::ivec_to_i32,
};

pub struct Compiler {
    interval: u64,
    pub threshold: u64,
    queues: VecDeque<(CodeHash, Bytecode)>,
    sled_db: Arc<RwLock<SledDB<QueryKeySlice>>>,
}

impl<'static> Compiler {
    pub fn new_with_db(
        interval: u64,
        threshold: u64,
        sled_db: Arc<RwLock<SledDB<QueryKeySlice>>>
    ) -> Self {
        Self {
            interval,
            threshold,
            queues: VecDeque::new(),
            sled_db,
        }
    }

    pub async fn routine_fn(&self) -> Result<()> {
        let start = Instant::now();
        let mut interval = interval_at(start, time::Duration::from_millis(self.interval));
        loop {
            interval.tick().await;
            let (code_hash, bytecode) = match self.queues.pop_front() {
                Some(item) => item,
                None => {
                    //empty queue
                    continue;
                }
            };
            let label = code_hash.update_prefix(KeyPrefix::SO).to_b256().to_string().leak();
            let so_path = Self::jit(label, bytecode.bytes_slice()).await?;
            let so_bytes = std::fs::read(&so_path)?;
            self.sled_db.write().unwrap().put(*item.as_inner(), &so_bytes, true).await?;
            println!("AOT Compiled for {label:#?}");
        }
    }

    pub async fn jit(label: &str, bytecode: &[u8]) -> Result<PathBuf> {
        let runtime_jit = RuntimeAot::new(AotCfg::default());
        runtime_jit.compile(label, bytecode).await
    }

    pub fn push_queue(&mut self, code_hash: CodeHash, bytecode: Bytecode) {
        self.queues.push_back((code_hash, bytecode));
    }
}
