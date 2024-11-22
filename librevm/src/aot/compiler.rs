use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use alloy_primitives::Bytes;
use revmc::eyre::Result;
use tokio::sync::mpsc;

use super::{SledDB, SledDBKeySlice};
use crate::{
    aot::{AotCfg, KeyPrefix, RuntimeAot, SledDbKey},
    runtime::get_runtime,
    storeutils::CodeHash,
};

// Control channels that send task tuple (code_hash, bytes) to worker
#[derive(Clone)]
pub struct CompilationQueue {
    pub threshold: u64,
    queue: mpsc::Sender<(CodeHash, Bytes)>,
}

impl CompilationQueue {
    pub fn new(threshold: u64, sled_db: Arc<RwLock<SledDB<SledDBKeySlice>>>) -> Self {
        let (queue, rx) = mpsc::channel::<(CodeHash, Bytes)>(100);
        let sled_db_clone = sled_db.clone();
        // start compiler worker
        let runtime = get_runtime();
        runtime.spawn(async move {
            let mut worker = CompileWorker::new(rx, sled_db_clone);
            worker.run().await;
        });

        Self { threshold, queue }
    }

    pub async fn push(&self, code_hash: CodeHash, bytecode: Bytes) {
        if let Err(err) = self.queue.send((code_hash, bytecode)).await {
            eprintln!("Failed to send to compilation queue: {:?}", err.to_string());
        }
    }
}

// Worker receives the tasks by channel, then compiles and saves in embedded DB
struct CompileWorker {
    queue_receiver: mpsc::Receiver<(CodeHash, Bytes)>,
    sled_db: Arc<RwLock<SledDB<SledDBKeySlice>>>,
}

impl CompileWorker {
    fn new(
        queue_receiver: mpsc::Receiver<(CodeHash, Bytes)>,
        sled_db: Arc<RwLock<SledDB<SledDBKeySlice>>>,
    ) -> Self {
        Self {
            queue_receiver,
            sled_db,
        }
    }

    async fn run(&mut self) {
        loop {
            if let Some((code_hash, bytecode)) = self.queue_receiver.recv().await {
                self.work(code_hash, bytecode).await;
            }
        }
    }

    async fn work(&self, code_hash: CodeHash, bytecode: Bytes) {
        let bytecode_slice = bytecode.to_vec();

        // skip if bytecode hash is zero hash
        if bytecode_slice.iter().all(|&b| b == 0) {
            return;
        }

        let key = SledDbKey::with_prefix(code_hash, KeyPrefix::SOPath);
        let label = key.to_b256().to_string().leak();
        let so_path = match Self::jit(label, &bytecode_slice) {
            Ok(path) => path,
            Err(err) => {
                eprintln!("Failed to JIT compile: {:?}", err.to_string());
                return;
            }
        };

        let result = {
            let sled_db = self.sled_db.write().unwrap();
            sled_db.put(*key.as_inner(), so_path.to_str().unwrap().as_bytes())
        };
        if let Err(err) = result {
            eprintln!("Failed to write in db: {:?}", err.to_string());
        }
    }

    pub fn jit(label: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let runtime_jit = RuntimeAot::new(AotCfg::default());
        runtime_jit.compile(label, bytecode)
    }
}
