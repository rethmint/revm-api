use revmc::eyre::{self, Result};
use sled::IVec;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

pub const SLEDDB_PATH: &str = "librevm/data";

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
        let db = SledDB::<K>::connect(SLEDDB_PATH).unwrap();

        Self {
            db: Arc::new(db),
            _marker: std::marker::PhantomData,
        }
    }

    fn connect(path: &str) -> Result<sled::Db> {
        sled::open(Path::new(path)).map_err(|e| eyre::Report::new(e))
    }

    pub fn put(&self, key: K, value: &[u8], flush: bool) -> Result<()> {
        self.db
            .insert(key, value)
            .map_err(|e| eyre::Report::new(e))?;

        if flush {
            self.db.flush().map_err(|e| eyre::Report::new(e))?;
        }

        Ok(())
    }

    pub fn get(&self, key: K) -> Result<Option<IVec>> {
        self.db.get(key).map_err(|e| eyre::Report::new(e))
    }

    pub fn key_iterator(&self) -> impl Iterator<Item = IVec> {
        self.db.iter().keys().filter_map(|res| res.ok())
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
