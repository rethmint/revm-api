use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    time,
};

use alloy_primitives::FixedBytes;
use revm::Database;
use revmc::eyre::Result;
use tokio::time::{interval_at, Instant};

use super::{QueryKeySlice, SledDB};
use crate::{
    aot::{AotCfg, KeyPrefix, RuntimeAot},
    gstorage::GoStorage,
    utils::ivec_to_i32,
};

pub const JIT_THRESHOLD: i32 = 0;

pub struct Compiler {
    // ms
    interval: u64,
    sled_db: Arc<RwLock<SledDB<QueryKeySlice>>>,
}

impl<'a> Compiler {
    pub fn new_with_db(interval: u64, sled_db: Arc<RwLock<SledDB<QueryKeySlice>>>) -> Self {
        Self { interval, sled_db }
    }

    pub async fn routine_fn(&self, mut kvstore: GoStorage<'a>) -> Result<()> {
        let start = Instant::now();
        let mut interval = interval_at(start, time::Duration::from_millis(self.interval));

        loop {
            interval.tick().await;

            let keys = {
                let db_read = self.sled_db.read().unwrap();
                db_read.count_keys_iter().collect::<Vec<_>>()
            };
            for mut key in keys {
                // skip empty bytecode (create tx)
                if key.to_b256().iter().all(|&byte| byte == 0) {
                    continue;
                }

                let count = {
                    let db_read = self.sled_db.read().unwrap();
                    let count_bytes = db_read.get(*key.as_inner()).unwrap_or(None);
                    count_bytes.and_then(|v| ivec_to_i32(&v)).unwrap_or(0)
                };

                if count > JIT_THRESHOLD {
                    key.update_prefix(KeyPrefix::SO);

                    // already aot compiled
                    if let Some(_) = { self.sled_db.read().unwrap().get(*key.as_inner())? } {
                        continue;
                    }

                    if let Ok(bytecode) =
                        kvstore.code_by_hash(FixedBytes::from_slice(key.as_slice()))
                    {
                        let label = key.to_b256().to_string().leak();
                        let so_path = Compiler::jit(label, &bytecode.original_byte_slice()).await?;
                        let so_bytes = std::fs::read(&so_path)?;

                        {
                            self.sled_db
                                .write()
                                .unwrap()
                                .put(*key.as_inner(), &so_bytes, true)?;
                        }
                        println!("AOT Compiled for {label:#?}");
                    }
                    continue;
                }
            }
        }
    }

    pub async fn jit(label: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let runtime_jit = RuntimeAot::new(AotCfg::default());
        runtime_jit.compile(label, bytecode).await
    }
}
