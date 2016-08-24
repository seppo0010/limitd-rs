use std::io;
use std::path::Path;

use leveldb;
use leveldb::database::Database as LDatabase;
use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use leveldb::snapshots::Snapshots;
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
    IOError(io::Error),
}

impl Error {
    fn is_level_db_error(&self) -> bool {
        match *self {
            Error::LevelDBError(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.is_level_db_error() && other.is_level_db_error()
    }
}

impl Eq for Error {}

impl From<leveldb::error::Error> for Error {
    fn from(e: leveldb::error::Error) -> Self {
        Error::LevelDBError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

pub trait Database: Send + Sync + 'static {
    fn put(&self, key: &[u8], value: &[u8]) -> Done<(), Error>;
    fn get(&self, key: &[u8]) -> Done<Option<Vec<u8>>, Error>;
    fn list(&self, key: &[u8]) -> Done<Vec<(Vec<u8>, Vec<u8>)>, Error>;
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

    fn list(&self, key: &[u8]) -> Done<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let read_opts = ReadOptions::new();
        // this is awful, but rust's leveldb lib does not seem to provide filtering for us
        Ok(self.db.snapshot().iter(read_opts).filter(|k| {
            K::as_slice(&k.0, |x| &x[..key.len()] == key)
        }).map(|k| (K::as_slice(&k.0, |x| x.to_vec()), k.1)).collect()).into_future()
    }
}

#[cfg(test)]
mod test{
    extern crate tempdir;

    use std::fmt;

    use futures::{Future, Task};
    use super::{Database, Key, LevelDB};
    use std::sync::Arc;

    pub fn assert_done<T, F>(mut f: F, result: Result<T::Item, T::Error>)
        where T: Future,
        T::Item: Eq + fmt::Debug,
        T::Error: Eq + fmt::Debug,
        F: FnMut() -> T,
    {
        let mut a = f();
        assert_eq!(&a.poll(&mut Task::new()).unwrap(), &result);
        drop(a);
    }

    #[test]
    fn put_get() {
        let db = Arc::new(LevelDB::<Key>::new(tempdir::TempDir::new("put_get").unwrap().path().to_str().unwrap()).unwrap());
        let key = [0, 1, 2, 3];
        let value = [4, 6, 8, 10];
        assert_done(move ||  {
            let db = db.clone();
            db.put(&key, &value).and_then(move |_| {
                db.get(&key)
            })
        }, Ok(Some(value.to_vec())))
    }

    #[test]
    fn put_put_get() {
        let db = Arc::new(LevelDB::<Key>::new(tempdir::TempDir::new("put_get").unwrap().path().to_str().unwrap()).unwrap());
        let key = [0, 1, 2, 3];
        let value = [4, 6, 8, 10];
        let value2 = [100, 101, 102, 103];
        assert_done(move ||  {
            let db = db.clone();
            db.put(&key, &value).and_then(move |_| {
                db.put(&key, &value2).and_then(move |_| {
                    db.get(&key)
                })
            })
        }, Ok(Some(value2.to_vec())))
    }

    #[test]
    fn get_none() {
        let db = LevelDB::<Key>::new(tempdir::TempDir::new("get_none").unwrap().path().to_str().unwrap()).unwrap();
        let key = [0, 1, 2, 3];
        assert_done(move ||  {
            db.get(&key)
        }, Ok(None));
    }

    #[test]
    fn put_list() {
        let db = Arc::new(LevelDB::<Key>::new(tempdir::TempDir::new("put_list").unwrap().path().to_str().unwrap()).unwrap());
        let key = [0, 1, 2, 3];
        let value = [4, 6, 8, 10];
        let key2 = [0, 1, 2, 3, 4];
        let value2 = [100, 101, 102, 103, 104];
        assert_done(move ||  {
            let db = db.clone();
            db.put(&key, &value).and_then(move |_| {
                db.put(&key2, &value2).and_then(move |_| {
                    db.list(&key)
                })
            })
        }, Ok(vec![(key.to_vec(), value.to_vec()), (key2.to_vec(), value2.to_vec())]));
    }
}
