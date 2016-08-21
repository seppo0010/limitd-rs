use database::Database;

pub struct HandlerData<D: Database> {
    db: D,
}

impl <D: Database> HandlerData<D> {
    pub fn new(db: D) -> Self {
        HandlerData {
            db: db,
        }
    }
}

#[derive(Clone)]
pub enum Method {
    Ping,
}

impl Method {
    fn handle_ping<ReqT: Req, ResT: Res, D: Database, Data: AsRef<HandlerData<D>>>(&self, _req: &ReqT, res: &mut ResT, _data: Data) {
        res.set_pong_response();
    }

    fn handle<ReqT: Req, ResT: Res, D: Database, Data: AsRef<HandlerData<D>>>(&self, req: &ReqT, res: &mut ResT, data: Data) {
        match *self {
            Method::Ping => self.handle_ping(req, res, data),
        }
    }
}

pub trait Req {
    fn method(&self) -> Method;
}

pub trait Res: Default {
    fn set_pong_response(&mut self);
}

pub fn handle<ReqT: Req, ResT: Res, D: Database, Data: AsRef<HandlerData<D>>>(req: &ReqT, res: &mut ResT, d: Data) {
    req.method().handle(req, res, d)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use futures::{IntoFuture, Done};

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
    }

    #[test]
    fn test_ping() {
        let request = Request::new(Method::Ping);
        let mut response = Response::default();
        let database = MockDatabase::default();
        let data = HandlerData::new(database);
        assert!(!response.pong_response);
        handle(&request, &mut response, Arc::new(data));
        assert!(response.pong_response);
    }
}
