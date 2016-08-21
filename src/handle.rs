use database::Database;

pub enum Method {
    Ping,
}

impl Method {
    fn handle_ping<ReqT: Req, ResT: Res, D: AsRef<Database>>(&self, _req: &ReqT, res: &mut ResT, _d: D) {
        res.set_pong_response();
    }

    fn handle<ReqT: Req, ResT: Res, D: AsRef<Database>>(&self, req: &ReqT, res: &mut ResT, d: D) {
        match *self {
            Method::Ping => self.handle_ping(req, res, d)
        }
    }
}

pub trait Req {
    fn method(&self) -> Method;
}

pub trait Res: Default {
    fn set_pong_response(&mut self);
}

pub fn handle<ReqT: Req, ResT: Res, D: AsRef<Database>>(req: &ReqT, res: &mut ResT, d: D) {
    req.method().handle(req, res, d)
}
