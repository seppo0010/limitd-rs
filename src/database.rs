use std::path::Path;

use leveldb;
use leveldb::database::Database as LDatabase;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use leveldb::kv::KV;
use db_key::Key as DBKey;

use futures::{IntoFuture, Done};

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
    fn put(&self, key: &[u8], value: &[u8]) -> Done<(), Error>;
    fn get(&self, key: &[u8]) -> Done<Option<Vec<u8>>, Error>;
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
    fn put(&self, key: &[u8], value: &[u8]) -> Done<(), Error> {
        let write_opts = WriteOptions::new();
        self.db.put(write_opts, K::from_u8(key), value).map_err(|e| Error::LevelDBError(e)).into_future()
    }

    fn get(&self, key: &[u8]) -> Done<Option<Vec<u8>>, Error> {
        let read_opts = ReadOptions::new();
        self.db.get(read_opts, K::from_u8(key)).map_err(|e| Error::LevelDBError(e)).into_future()
    }
}

#[cfg(test)]
mod test{
    extern crate tempdir;

    use futures::Future;
    use super::{Database, Key, LevelDB};

    #[test]
    fn put_get() {
        let db = LevelDB::<Key>::new(tempdir::TempDir::new("put_get").unwrap().path().to_str().unwrap()).unwrap();
        let key = [0, 1, 2, 3];
        let value = [4, 6, 8, 10];
        db.put(&key, &value).and_then(move |_| {
            db.get(&key).map(move |v| {
                assert_eq!(v.unwrap(), &value);
            })
        });
    }

    #[test]
    fn put_put_get() {
        let db = LevelDB::<Key>::new(tempdir::TempDir::new("put_get").unwrap().path().to_str().unwrap()).unwrap();
        let key = [0, 1, 2, 3];
        let value = [4, 6, 8, 10];
        let value2 = [100, 101, 102, 103];
        db.put(&key, &value).and_then(move |_| {
            db.put(&key, &value2).and_then(move |_| {
                db.get(&key).map(move |v| {
                    assert_eq!(v.unwrap(), &value2);
                })
            })
        });
    }

    #[test]
    fn get_none() {
        let db = LevelDB::<Key>::new(tempdir::TempDir::new("get_none").unwrap().path().to_str().unwrap()).unwrap();
        let key = [0, 1, 2, 3];
        db.get(&key).map(move |v| {
            assert!(v.is_none());
        });
    }
}
