use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, RwLock},
    time,
};

use alloy_primitives::Bytes;
use revmc::eyre::Result;
use tokio::time::{interval_at, Instant};

use super::{QueryKeySlice, SledDB};
use crate::{
    aot::{AotCfg, KeyPrefix, QueryKey, RuntimeAot},
    storeutils::CodeHash,
};

pub struct Compiler {
    interval: u64,
    pub threshold: u64,
    queue: RwLock<VecDeque<(CodeHash, Bytes)>>,
    sled_db: Arc<RwLock<SledDB<QueryKeySlice>>>,
}

impl Compiler {
    pub fn new_with_db(
        interval: u64,
        threshold: u64,
        sled_db: Arc<RwLock<SledDB<QueryKeySlice>>>,
    ) -> Self {
        Self {
            interval,
            threshold,
            queue: RwLock::new(VecDeque::new()),
            sled_db,
        }
    }

    pub async fn routine_fn(&mut self) -> Result<()> {
        let start = Instant::now();
        let mut interval = interval_at(start, time::Duration::from_millis(self.interval));

        loop {
            interval.tick().await;

            let queue_front = {
                let mut queue = self.queue.write().unwrap();
                queue.pop_front()
            };
            let (code_hash, bytecode) = match queue_front {
                Some(item) => item,
                None => {
                    //empty queue
                    continue;
                }
            };

            let bytecode_slice = bytecode.to_vec();
            // skip if bytecode hash is zero hash
            if bytecode_slice.iter().all(|&b| b == 0) {
                continue;
            }

            let key = QueryKey::with_prefix(code_hash, KeyPrefix::SOPath);

            let label = key.to_b256().to_string().leak();
            let so_path = match Self::jit(label, &bytecode_slice) {
                Ok(path) => path,
                Err(err) => {
                    // retry in next interval
                    eprintln!("Failed to JIT compile: {:?}", err);
                    self.push_queue(code_hash, bytecode);
                    continue;
                }
            };

            let result = {
                let sled_db = self.sled_db.write().unwrap();
                sled_db.put(*key.as_inner(), so_path.to_str().unwrap().as_bytes())
            };
            if let Err(err) = result {
                eprintln!("Failed to write in db: {:?}", err);
                self.push_queue(code_hash, bytecode);
            }
        }
    }

    pub fn jit(label: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let runtime_jit = RuntimeAot::new(AotCfg::default());
        runtime_jit.compile(label, bytecode)
    }

    pub fn push_queue(&mut self, code_hash: CodeHash, bytecode: Bytes) {
        let mut queue = self.queue.write().unwrap();
        queue.push_back((code_hash, bytecode));
    }
}
