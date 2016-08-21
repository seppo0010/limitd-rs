mod pb;

use std::io;
use std::sync::Arc;

use database::Database;
use server::Server;

use self::pb::serve_protobuf;

pub enum Protocol {
    ProtocolBuffer,
    Avro,
}

impl Protocol {
    pub fn serve<D: Database>(&self, server: &mut Server, d: Arc<D>) -> io::Result<()> {
        match *self {
            Protocol::ProtocolBuffer => server.serve(move |r| serve_protobuf::<D>(r, d.clone())),
            _ => unimplemented!(),
        }
    }
}
