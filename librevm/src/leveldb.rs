use leveldb::database::Database;
use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use tempdir::TempDir;

fn _datababse() {
    let tempdir = TempDir::new("Rethmint").unwrap();
    let path = tempdir.path();

    let mut options = Options::new();
    options.create_if_missing = true;
    let database = match Database::open(path, options) {
        Ok(db) => db,
        Err(e) => {
            panic!("failed to open database: {:?}", e)
        }
    };

    let write_opts = WriteOptions::new();
    match database.put(write_opts, 1, &[1]) {
        Ok(_) => (),
        Err(e) => {
            panic!("failed to write to database: {:?}", e)
        }
    };

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 1);

    match res {
        Ok(data) => {
            assert!(data.is_some());
            assert_eq!(data, Some(vec![1]));
        }
        Err(e) => {
            panic!("failed reading data: {:?}", e)
        }
    }

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);
    let entry = iter.next();
    assert_eq!(entry, Some((1, vec![1])));
}
