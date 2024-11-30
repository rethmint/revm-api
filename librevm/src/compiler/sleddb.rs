use revmc::eyre::{self, Result};
use sled::IVec;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

use super::path::sleddb_path;

pub struct SledDB<K>
where
    K: AsRef<[u8]>,
{
    pub db: Arc<sled::Db>,
    _marker: std::marker::PhantomData<K>,
}

impl<K> SledDB<K>
where
    K: AsRef<[u8]>,
{
    pub fn init() -> Self {
        let db = SledDB::<K>::connect(sleddb_path().to_str().unwrap()).unwrap();

        Self {
            db: Arc::new(db),
            _marker: std::marker::PhantomData,
        }
    }

    fn connect(path: &str) -> Result<sled::Db> {
        sled::open(Path::new(path)).map_err(|e| eyre::Report::new(e))
    }

    pub fn put(&self, key: K, value: &[u8]) -> Result<()> {
        self.db
            .insert(key, value)
            .map_err(|e| eyre::Report::new(e))?;

        self.db.flush().map_err(|e| eyre::Report::new(e))?;

        Ok(())
    }

    pub fn get(&self, key: K) -> Result<Option<IVec>> {
        self.db.get(key).map_err(|e| eyre::Report::new(e))
    }
}

impl<K> Clone for SledDB<K>
where
    K: AsRef<[u8]>,
{
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
            _marker: PhantomData,
        }
    }
}
