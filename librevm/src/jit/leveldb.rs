use db_key::Key;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use revmc::eyre::{self, Result};
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

pub fn init_db() -> Arc<Database<i32>> {
    let db_path = "Path";
    let db = LevelDB::<i32>::connect(db_path).unwrap();
    Arc::new(db)
}

pub struct LevelDB<'a, K>
where
    K: 'a + Key,
{
    pub db: Arc<Database<K>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, K> LevelDB<'a, K>
where
    K: 'a + Key,
{
    pub fn connect(path: &str) -> Result<Database<i32>> {
        let mut options = Options::new();
        options.create_if_missing = true;

        let db = match Database::open(Path::new(path), options) {
            Ok(db) => db,
            Err(_) => {
                panic!();
            }
        };

        let write_opts = WriteOptions::new();
        match db.put(write_opts, 1, &[1]) {
            Ok(_) => (),
            Err(e) => {
                panic!()
            }
        };

        Ok(db)
    }

    pub fn put(&self, key: K, value: &[u8]) -> Result<()> {
        let write_options = WriteOptions::new();
        self.db
            .put(write_options, key, value)
            .map_err(|e| eyre::Report::new(e))
    }

    pub fn get(&self, key: K) -> Result<Option<Vec<u8>>> {
        let read_options = ReadOptions::new();
        self.db
            .get(read_options, key)
            .map_err(|e| eyre::Report::new(e))
    }
}
