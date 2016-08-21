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
