mod pb;

use std::io;

use futures::{Finished, finished};

use server::Server;

use self::pb::request::Request;
use self::pb::response::Response;

pub enum Protocol {
    ProtocolBuffer,
    Avro,
}

fn serve_protobuf(r: Request) -> Finished<Response, io::Error> {
    let mut response = Response::new();
    response.set_request_id(r.get_id().to_owned());
    finished(response)
}

impl Protocol {
    pub fn serve(&self, server: &mut Server) -> io::Result<()> {
        match *self {
            Protocol::ProtocolBuffer => server.serve(serve_protobuf),
            _ => unimplemented!(),
        }
    }
}
