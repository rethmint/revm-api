use std::{path::PathBuf, sync::Arc, time};

use alloy_primitives::{hex, FixedBytes};
use revm::Database;
use revmc::eyre::{Context, Result};
use tokio::time::{interval_at, Instant};

use super::{QueryKeySlice, SledDB};
use crate::{
    aot::{AotCfg, KeyPrefix, QueryKey, RuntimeAot},
    gstorage::GoStorage,
    utils::ivec_to_i32,
};

pub const JIT_THRESHOLD: i32 = 0;

pub struct Cronner {
    // ms
    interval: u64,
    sled_db: Arc<SledDB<QueryKeySlice>>,
}

impl<'a> Cronner {
    pub fn new_with_db(interval: u64, sled_db: Arc<SledDB<QueryKeySlice>>) -> Self {
        Self { interval, sled_db }
    }

    pub async fn routine_fn(&self, mut kvstore: GoStorage<'a>) -> Result<()> {
        let interval = self.interval.clone();
        let sled_db = self.sled_db.clone();

        let start = Instant::now();
        let mut interval = interval_at(start, time::Duration::from_millis(interval));

        loop {
            interval.tick().await;

            for mut key in sled_db
                .key_iterator()
                .filter_map(|iv| {
                    let k = QueryKey::from_ivec(iv);

                    if k.match_prefix(KeyPrefix::Count) {
                        Some(k)
                    } else {
                        None
                    }
                })
                .into_iter()
            {
                let count_bytes = sled_db.get(*key.as_inner()).unwrap_or(None);
                let count = count_bytes.and_then(|v| ivec_to_i32(&v)).unwrap_or(1);

                if count > JIT_THRESHOLD {
                    if key.to_b256().iter().all(|&byte| byte == 0) {
                        continue;
                    }

                    if let Ok(bytecode) =
                        kvstore.code_by_hash(FixedBytes::from_slice(key.as_slice()))
                    {
                        //println!("Bytecode: {:#02X?}", &bytecode.original_byte_slice()[..10]);
                        //println!("Bytecode hash, {:#?}", key.to_b256());

                        //let bytecode_hash = key.to_b256();
                        //let bytes = hex::decode(bytecode_hash).unwrap();
                        //let label = String::from_utf8(bytes).unwrap().leak();
                        let label = "afn";

                        let so_path = Cronner::jit(label, &bytecode.original_byte_slice()).await?;
                        key.update_prefix(KeyPrefix::SO);

                        let so_bytes = std::fs::read(&so_path)?;
                        sled_db.put(*key.as_inner(), &so_bytes, true)?;
                        println!("Success jit!");
                    }
                    continue;
                }
            }
        }
    }

    pub async fn jit(label: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let runtime_jit = RuntimeAot::new(AotCfg::default());
        runtime_jit
            .compile(label, bytecode)
            .await
            .wrap_err("Compilation fail")
    }
}
