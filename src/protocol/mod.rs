mod pb;

use std::io;
use std::sync::Arc;

use database::Database;
use handle::{HandlerData, TimeGenerator};
use server::Server;

use self::pb::serve_protobuf;

pub enum Protocol {
    ProtocolBuffer,
    Avro,
}

impl Protocol {
    pub fn serve<D: Database, T: TimeGenerator>(&self, server: &mut Server, d: Arc<HandlerData<D, T>>) -> io::Result<()> {
        match *self {
            Protocol::ProtocolBuffer => server.serve(move |r| serve_protobuf::<D, T>(r, d.clone())),
            _ => unimplemented!(),
        }
    }
}
