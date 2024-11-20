use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use revm::primitives::Bytecode;
use revmc::eyre::Result;

use super::{QueryKeySlice, SledDB};
use crate::{
    aot::{AotCfg, KeyPrefix, QueryKey, RuntimeAot},
    storeutils::CodeHash,
};

pub struct Compiler {
    interval: u64,
    pub threshold: u64,
    queue: RwLock<VecDeque<(CodeHash, Bytecode)>>,
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

    pub fn routine_fn(&mut self) -> Result<()> {
        let start = Instant::now();
        let mut next_tick = start + Duration::from_millis(self.interval);

        loop {
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

            let bytecode_slice = bytecode.bytes_slice();

            // skip create
            if bytecode_slice.iter().all(|&b| b == 0) {
                continue;
            }

            let key = QueryKey::with_prefix(code_hash, KeyPrefix::SOPath);

            {
                let db_read = self.sled_db.read().unwrap();

                // skip already compiled
                if let Some(_) = db_read.get(*key.as_inner())? {
                    continue;
                }
            }

            let label = key.to_b256().to_string().leak();
            let so_path = Self::jit(label, bytecode_slice)?;

            self.sled_db.write().unwrap().put(
                *key.as_inner(),
                so_path.to_str().unwrap().as_bytes(),
                true,
            )?;

            println!("AOT Compiled for {label:#?}");

            let now = Instant::now();
            if now < next_tick {
                thread::sleep(next_tick - now);
            }
            next_tick += Duration::from_millis(self.interval);
        }
    }

    pub fn jit(label: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let runtime_jit = RuntimeAot::new(AotCfg::default());
        runtime_jit.compile(label, bytecode)
    }

    pub fn push_queue(&mut self, code_hash: CodeHash, bytecode: Bytecode) {
        let mut queue = self.queue.write().unwrap();
        queue.push_back((code_hash, bytecode));
    }
}
