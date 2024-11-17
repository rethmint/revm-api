use std::{path::PathBuf, sync::Arc, time};

use alloy_primitives::FixedBytes;
use revm::Database;
use revmc::eyre::{Context, Result};
use tokio::time::{interval_at, Instant};

use super::{QueryKeySlice, SledDB};
use crate::{
    gstorage::GoStorage,
    jit::{JitCfg, JitUnit, KeyPrefix, QueryKey, RuntimeJit},
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

    pub async fn routine_fn(&self, mut kvstore: GoStorage<'a>) {
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
                let count = count_bytes.as_ref().map_or(1, |v| {
                    let bytes: [u8; 4] = v.to_vec().as_slice().try_into().unwrap_or([0, 0, 0, 0]);
                    i32::from_be_bytes(bytes)
                });

                if count > JIT_THRESHOLD {
                    let prefix_zeros = &key.to_b256()[0..10];
                    if prefix_zeros.iter().all(|&byte| byte == 0) {
                        continue;
                    }

                    // use gostorage db
                    if let Ok(bytecode) =
                        kvstore.code_by_hash(FixedBytes::from_slice(&*key.to_b256().as_slice()))
                    {
                        //let bytecode_hash = key.to_b256();
                        //let bytes = hex::decode(bytecode_hash).unwrap();
                        //let label = String::from_utf8(bytes).unwrap().leak();
                        let label = "afn";

                        match Cronner::jit(label, &bytecode.original_byte_slice()).await {
                            Ok(so_path) => {
                                println!("Success jit!");

                                key.update_prefix(KeyPrefix::SO);

                                match std::fs::read(&so_path) {
                                    Ok(so_bytes) => {
                                        sled_db.put(*key.as_inner(), &so_bytes, true).unwrap()
                                    }
                                    Err(err) => println!("While jit: {:#?}", err),
                                }
                            }
                            Err(err) => println!("While jit: {:#?}", err),
                        }
                    }
                    continue;
                }
            }
        }
    }

    pub async fn jit(label: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let unit = JitUnit::new(label, 70);
        let runtime_jit = RuntimeJit::new(unit, JitCfg::default());
        runtime_jit
            .compile(bytecode)
            .await
            .wrap_err("Compilation fail")
    }
}
