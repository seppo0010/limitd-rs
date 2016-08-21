use std::path::Path;

use leveldb;
use leveldb::database::Database as LDatabase;
use leveldb::options::{Options, WriteOptions};
use leveldb::kv::KV;
use db_key::Key as DBKey;

use futures::{failed, finished, Future};

// Not quite a fan of this implementation
pub struct Key {
    data: Vec<u8>,
}

impl DBKey for Key {
    fn from_u8(key: &[u8]) -> Self {
        Key { data: key.to_vec() }
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(&*self.data)
    }
}

#[derive(Debug)]
pub enum Error {
    LevelDBError(leveldb::error::Error),
}

impl From<leveldb::error::Error> for Error {
    fn from(e: leveldb::error::Error) -> Self {
        Error::LevelDBError(e)
    }
}

pub trait Database: Send + Sync + 'static {
    fn put(&self, key: &[u8], value: &[u8]) -> Box<Future<Item=(), Error=Error>>;
}

pub struct LevelDB<K: DBKey> {
    db: LDatabase<K>,
}

impl<K: DBKey> LevelDB<K> {
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut opts = Options::new();
        opts.create_if_missing = true;
        let db = try!(LDatabase::open(Path::new(path), opts));
        Ok(LevelDB { db: db })
    }
}

impl<K: DBKey + 'static> Database for LevelDB<K> {
    fn put(&self, key: &[u8], value: &[u8]) -> Box<Future<Item=(), Error=Error>> {
        let write_opts = WriteOptions::new();
        match self.db.put(write_opts, K::from_u8(key), value) {
            Ok(_) => { return finished(()).boxed() },
            Err(e) => { return failed(Error::LevelDBError(e)).boxed() }
        }
    }
}
