use std::sync::Arc;

use futures::{BoxFuture, IntoFuture, Future, failed};

use bucket::Buckets;
use database::{Database, Error};

pub struct HandlerData<D: Database> {
    db: D,
    buckets: Buckets,
}

impl <D: Database> HandlerData<D> {
    pub fn new(db: D, buckets: Buckets) -> Self {
        HandlerData {
            db: db,
            buckets: buckets,
        }
    }
}

#[derive(Clone)]
pub enum Method {
    Ping,
    Status,
}

impl Method {
    fn handle_status<ReqT: Req, ResT: Res + 'static, D: Database>(&self, req: &ReqT, mut res: ResT, data: Arc<HandlerData<D>>) -> BoxFuture<ResT, Error> {
        let d = data.clone();
        let key = req.key();
        let bucket = data.buckets.get(&*req.bucket());
        match bucket {
            Some(b) => b.status(&*key, &d.db).map(|i| {
                res.set_status_response(i.into_iter());
                res
            }).boxed(),
            None => failed(Error::InvalidBucket).boxed()
        }
    }

    fn handle_ping<ReqT: Req, ResT: Res + 'static, D: Database>(&self, _req: &ReqT, mut res: ResT, _data: Arc<HandlerData<D>>) -> BoxFuture<ResT, Error> {
        res.set_pong_response();
        Ok(res).into_future().boxed()
    }

    fn handle<ReqT: Req, ResT: Res + 'static, D: Database>(&self, req: &ReqT, res: ResT, data: Arc<HandlerData<D>>) -> BoxFuture<ResT, Error> {
        match *self {
            Method::Ping => self.handle_ping(req, res, data),
            Method::Status => self.handle_status(req, res, data),
        }
    }
}

pub trait Req: Sync {
    fn method(&self) -> Method;
    fn bucket(&self) -> String;
    fn key(&self) -> String;
}

pub trait Res: Default + Send {
    fn set_pong_response(&mut self);
    fn set_status_response<I: Iterator<Item=(String, i32, i32, i32)>>(&mut self, items: I);
}

pub fn handle<ReqT: Req, ResT: Res + 'static, D: Database>(req: &ReqT, res: ResT, d: Arc<HandlerData<D>>) -> BoxFuture<ResT, Error> {
    req.method().handle(req, res, d)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fmt;
    use std::sync::{Arc, Mutex};

    use futures::{Future, Task, IntoFuture, Done};

    use bucket::{Bucket, Buckets};
    use database::{Database, Error};

    use super::{Method, Req, Res, HandlerData, handle};

    struct Request {
        method: Method,
        bucket: Option<String>,
        key: Option<String>,
    }

    impl Request {
        fn new(method: Method, bucket: Option<String>, key: Option<String>) -> Self {
            Request {
                method: method,
                bucket: bucket,
                key: key,
            }
        }
    }

    impl Req for Request {
        fn method(&self) -> Method { self.method.clone() }
        fn bucket(&self) -> String { self.bucket.clone().unwrap() }
        fn key(&self) -> String { self.key.clone().unwrap() }
    }

    #[derive(Default)]
    struct Response {
        pong_response: bool,
        status_response: Option<Vec<(String, i32, i32, i32)>>,
    }

    impl Res for Response {
        fn set_pong_response(&mut self) {
            self.pong_response = true;
        }

        fn set_status_response<I: Iterator<Item=(String, i32, i32, i32)>>(&mut self, items: I) {
            self.status_response = Some(items.collect());
        }
    }

    #[derive(Default)]
    struct MockDatabase {
        data: Mutex<HashMap<Vec<u8>, Vec<u8>>>,
    }

    impl Database for MockDatabase {
        fn put(&self, key: &[u8], value: &[u8]) -> Done<(), Error> {
            self.data.lock().unwrap().insert(key.to_vec(), value.to_vec());
            Ok(()).into_future()
        }

        fn get(&self, key: &[u8]) -> Done<Option<Vec<u8>>, Error> {
            Ok(self.data.lock().unwrap().get(key).cloned()).into_future()
        }

        fn list(&self, key: &[u8]) -> Done<Vec<(Vec<u8>, Vec<u8>)>, Error> {
            Ok(self.data.lock().unwrap().iter().filter(|k| {
                k.0.len() >= key.len() && &k.0[..key.len()] == key
            }).map(|k| (k.0.to_vec(), k.1.clone())).collect()).into_future()
        }
    }

    // copying and pasting this? not great
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
    fn test_ping() {
        assert_done(move || {
            let request = Request::new(Method::Ping, None, None);
            let database = MockDatabase::default();
            let data = HandlerData::new(database, Buckets::default());
            let response = Response::default();
            assert!(!response.pong_response);
            handle(&request, response, Arc::new(data)).map(|r| {
                r.pong_response
            })
        }, Ok(true));
    }

    #[test]
    fn test_status() {
        let k = "key123";
        assert_done(move || {
            let request = Request::new(Method::Status, Some("bucket".to_owned()), Some("{\"content\":123,\"lastDrip\":1234567890000}".to_owned()));

            let database = MockDatabase::default();
            database.put(k.as_bytes(), "hello".as_bytes());

            let mut buckets = Buckets::default();
            let bucket = Bucket::new("bucket".to_owned(), 1, 1, 1);
            buckets.add(bucket);

            let data = HandlerData::new(database, buckets);

            let response = Response::default();
            assert!(!response.pong_response);
            handle(&request, response, Arc::new(data)).map(|r| {
                r.status_response
            })
        }, Ok(Some(vec![(k.to_owned(), 0, 0, 0)])));
    }
}
