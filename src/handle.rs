use futures::{BoxFuture, IntoFuture, Future};

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
    fn handle_status<ReqT: Req, ResT: Res + 'static, D: Database, Data: AsRef<HandlerData<D>>>(&self, _req: &ReqT, mut res: ResT, data: Data) -> BoxFuture<ResT, Error> {
        data.as_ref().db.list(&[]).map(move |_| {
            res.set_pong_response();
            res
        }).boxed()
    }

    fn handle_ping<ReqT: Req, ResT: Res + 'static, D: Database, Data: AsRef<HandlerData<D>>>(&self, _req: &ReqT, mut res: ResT, _data: Data) -> BoxFuture<ResT, Error> {
        res.set_pong_response();
        Ok(res).into_future().boxed()
    }

    fn handle<ReqT: Req, ResT: Res + 'static, D: Database, Data: AsRef<HandlerData<D>>>(&self, req: &ReqT, res: ResT, data: Data) -> BoxFuture<ResT, Error> {
        match *self {
            Method::Ping => self.handle_ping(req, res, data),
            Method::Status => self.handle_status(req, res, data),
        }
    }
}

pub trait Req {
    fn method(&self) -> Method;
}

pub trait Res: Default + Send {
    fn set_pong_response(&mut self);
}

pub fn handle<ReqT: Req, ResT: Res + 'static, D: Database, Data: AsRef<HandlerData<D>>>(req: &ReqT, res: ResT, d: Data) -> BoxFuture<ResT, Error> {
    req.method().handle(req, res, d)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fmt;
    use std::sync::{Arc, Mutex};

    use futures::{Future, Task, IntoFuture, Done};

    use bucket::Buckets;
    use database::{Database, Error};

    use super::{Method, Req, Res, HandlerData, handle};

    struct Request {
        method: Method,
    }

    impl Request {
        fn new(method: Method) -> Self {
            Request {
                method: method,
            }
        }
    }

    impl Req for Request {
        fn method(&self) -> Method { self.method.clone() }
    }

    #[derive(Default)]
    struct Response {
        pong_response: bool,
    }

    impl Res for Response {
        fn set_pong_response(&mut self) {
            self.pong_response = true;
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

        fn list(&self, key: &[u8]) -> Done<Vec<Vec<u8>>, Error> {
            Ok(self.data.lock().unwrap().iter().filter(|k| { &k.0[..key.len()] == key }).map(|k| k.1.clone()).collect()).into_future()
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
            let request = Request::new(Method::Ping);
            let database = MockDatabase::default();
            let data = HandlerData::new(database, Buckets::default());
            let response = Response::default();
            assert!(!response.pong_response);
            handle(&request, response, Arc::new(data)).map(|r| {
                r.pong_response
            })
        }, Ok(true));
    }
}
