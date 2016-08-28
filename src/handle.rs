use std::sync::Arc;

use futures::{BoxFuture, IntoFuture, Future, failed};

use bucket::Buckets;
use database::{Database, Error};
use time;

pub trait TimeGenerator: Send + Sync + 'static {
    fn now(&self) -> time::Tm;
}

pub struct DefaultTimeGenerator;

impl TimeGenerator for DefaultTimeGenerator {
    fn now(&self) -> time::Tm {
        time::now_utc()
    }
}

pub struct HandlerData<D: Database, T: TimeGenerator> {
    db: D,
    buckets: Buckets,
    time: T,
}

impl <D: Database, T: TimeGenerator> HandlerData<D, T> {
    pub fn new_t(db: D, buckets: Buckets, time: T) -> Self {
        HandlerData {
            db: db,
            buckets: buckets,
            time: time,
        }
    }
}

impl <D: Database> HandlerData<D, DefaultTimeGenerator> {
    pub fn new(db: D, buckets: Buckets) -> Self {
        HandlerData::new_t(db, buckets, DefaultTimeGenerator)
    }
}

#[derive(Clone)]
pub enum Method {
    Ping,
    Status,
}

impl Method {
    fn handle_status<ReqT: Req, ResT: Res + 'static, D: Database, T: TimeGenerator>(&self, req: &ReqT, mut res: ResT, data: Arc<HandlerData<D, T>>) -> BoxFuture<ResT, Error> {
        let d = data.clone();
        let key = req.key();
        let bucket = data.buckets.get(&*req.bucket());
        match bucket {
            Some(b) => b.status(&*key, &d.time.now(), &d.db).map(|i| {
                res.set_status_response(i.into_iter());
                res
            }).boxed(),
            None => failed(Error::InvalidBucket).boxed()
        }
    }

    fn handle_ping<ReqT: Req, ResT: Res + 'static, D: Database, T: TimeGenerator>(&self, _req: &ReqT, mut res: ResT, _data: Arc<HandlerData<D, T>>) -> BoxFuture<ResT, Error> {
        res.set_pong_response();
        Ok(res).into_future().boxed()
    }

    fn handle<ReqT: Req, ResT: Res + 'static, D: Database, T: TimeGenerator>(&self, req: &ReqT, res: ResT, data: Arc<HandlerData<D, T>>) -> BoxFuture<ResT, Error> {
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

pub fn handle<ReqT: Req, ResT: Res + 'static, D: Database, T: TimeGenerator>(req: &ReqT, res: ResT, d: Arc<HandlerData<D, T>>) -> BoxFuture<ResT, Error> {
    req.method().handle(req, res, d)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fmt;
    use std::sync::{Arc, Mutex};
    use time;

    use futures::{Future, Task, IntoFuture, Done};

    use bucket::{Bucket, Buckets};
    use database::{Database, Error};

    use super::{Method, Req, Res, HandlerData, handle, TimeGenerator};

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

    struct MockTimeGenerator {
        time: Mutex<Vec<time::Tm>>,
    }

    impl MockTimeGenerator {
        fn new(offsets: Vec<i64>) -> MockTimeGenerator {
            let base = time::at(time::Timespec::new(1234567890, 0));
            MockTimeGenerator {
                time: Mutex::new(offsets.into_iter().map(|o| base + time::Duration::seconds(o)).collect()),
            }
        }
    }

    impl TimeGenerator for MockTimeGenerator {
        fn now(&self) -> time::Tm {
            self.time.lock().unwrap().remove(0)
        }
    }

    impl Drop for MockTimeGenerator {
        fn drop(&mut self) {
            // just checking that all times were used, no involuntary time shift
            // assert!(self.time.lock().unwrap().len() == 0);
        }
    }

    #[derive(Default)]
    struct MockDatabase {
        data: Mutex<HashMap<Vec<u8>, Vec<u8>>>,
    }

    impl Database for MockDatabase {
        fn put(&self, bucket: &[u8], key: &[u8], value: &[u8]) -> Done<(), Error> {
            let mut k = bucket.to_vec();
            k.push(":".as_bytes()[0]);
            k.extend(key);
            self.data.lock().unwrap().insert(k, value.to_vec());
            Ok(()).into_future()
        }

        fn get(&self, bucket: &[u8], key: &[u8]) -> Done<Option<Vec<u8>>, Error> {
            let mut k = bucket.to_vec();
            k.push(":".as_bytes()[0]);
            k.extend(key);
            Ok(self.data.lock().unwrap().get(&*k).cloned()).into_future()
        }

        fn list(&self, bucket: &[u8], key: &[u8]) -> Done<Vec<(Vec<u8>, Vec<u8>)>, Error> {
            let mut bk = bucket.to_vec();
            bk.push(":".as_bytes()[0]);
            bk.extend(key);
            Ok(self.data.lock().unwrap().iter().filter(|k| {
                k.0.len() >= bk.len() && &k.0[..bk.len()] == &*bk
            }).map(|k| (k.0[1 + bucket.len()..].to_vec(), k.1.clone())).collect()).into_future()
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
            let bucket_name = "bucket".to_owned();
            let request = Request::new(Method::Status, Some(bucket_name.clone()), Some(k.to_owned()));

            let timer = MockTimeGenerator::new(vec![0, 3]);
            let now = timer.now().to_timespec().sec;

            let database = MockDatabase::default();
            database.put(bucket_name.as_bytes(), k.as_bytes(), format!("{{\"content\":0,\"lastDrip\":{}}}", now * 1000).as_bytes());

            let mut buckets = Buckets::default();
            let bucket = Bucket::new("bucket".to_owned(), 1000, 1, 10);
            buckets.add(bucket);

            let data = HandlerData::new_t(database, buckets, timer);

            let response = Response::default();
            assert!(!response.pong_response);

            handle(&request, response, Arc::new(data)).map(|r| {
                r.status_response
            })
        }, Ok(Some(vec![(k.to_owned(), 3, 1234567900, 10)])));
    }
}
