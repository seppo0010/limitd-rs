use std::io;
use std::path::Path;

use leveldb;
use leveldb::database::Database as LDatabase;
use leveldb::options::{Options, WriteOptions};
use leveldb::kv::KV;
use db_key::Key;

use futures::{failed, finished, Future};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    LevelDBError(leveldb::error::Error),
}

pub trait Database: Send + Sync + 'static {
    fn put(&self, key: &str, value: &[u8]) -> Box<Future<Item=(), Error=Error>>;
}

pub struct LevelDB {
    path: String,
}

impl LevelDB {
    pub fn new(path: String) -> Result<Self, Error> {
        Ok(LevelDB { path: path })
    }
}

impl Database for LevelDB {
    fn put(&self, key: &[u8], value: &[u8]) -> Box<Future<Item=(), Error=Error>> {
        let mut opts = Options::new();
        opts.create_if_missing = true;
        let db = match LDatabase::open(Path::new(&*self.path), opts) {
            Ok(db) => Box::new(db),
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        let write_opts = WriteOptions::new();
        match db.put(write_opts, key, value) {
            Ok(_) => { return finished(()).boxed() },
            Err(e) => { return failed(Error::LevelDBError(e)).boxed() }
        }
    }
}
