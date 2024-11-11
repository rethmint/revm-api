use std::{future::Future, sync::Arc, time};

use revmc::eyre::{Context, Result};
use tokio::time::{interval_at, Instant};

use super::{QueryKeySlice, SledDB};
use crate::jit::{JitCfg, JitUnit, KeyPrefix, QueryKey, RuntimeJit};

pub const JIT_THRESHOLD: i32 = 1;

pub struct Cronner {
    // ms
    interval: u64,
    sled_db: Arc<SledDB<QueryKeySlice>>,
}

impl Cronner {
    pub fn new_with_db(interval: u64, sled_db: Arc<SledDB<QueryKeySlice>>) -> Self {
        Self { interval, sled_db }
    }

    pub fn routine_fn(&self) -> impl Future<Output = ()> + Send + 'static {
        let interval = self.interval.clone();
        let sled_db = self.sled_db.clone();

        async move {
            Cronner::cron(interval, sled_db).await;
        }
    }

    pub async fn cron(interval: u64, sled_db: Arc<SledDB<QueryKeySlice>>) {
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

                    key.update_prefix(KeyPrefix::Bytecode);
                    if let Some(bytecode) = sled_db.get(*key.as_inner()).unwrap_or(None) {
                        let bytecode_hash = key.to_b256();
                        // leak for cast to static
                        let label = Cronner::mangle_hex(bytecode_hash.as_slice()).leak();

                        key.update_prefix(KeyPrefix::Label);
                        if let None = sled_db.get(*key.as_inner()).unwrap_or(None) {
                            match Cronner::jit(label, &bytecode) {
                                Ok(_) => {
                                    println!("Success jit!");

                                    sled_db
                                        .put(*key.as_inner(), label.as_bytes(), true)
                                        .unwrap()
                                }
                                Err(err) => println!("While jit: {:#?}", err),
                            }
                        }
                    }
                    continue;
                }
            }
        }
    }

    pub fn jit(label: &'static str, bytecode: &[u8]) -> Result<()> {
        let unit = JitUnit::new(label, bytecode.to_vec(), 70);
        let runtime_jit = RuntimeJit::new(unit, JitCfg::default());
        runtime_jit.compile().wrap_err("Compilation fail")
    }

    fn mangle_hex(hex: &[u8]) -> String {
        let hex_part: String = hex
            .iter()
            .take(3)
            .map(|byte| format!("{:02x}", byte))
            .collect();

        format!("_{}", hex_part)
    }
}
