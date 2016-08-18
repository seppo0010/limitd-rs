extern crate env_logger;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate futures_io;
extern crate futures_mio;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate time;

mod protocol_protobuf;
mod io2;

use std::env;
use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use futures::Future;
use futures::stream::Stream;
use futures_io::{copy, TaskIo};
use getopts::Options;
use log::LogLevel;
use protobuf::parse_from_bytes;
use protobuf::Message;
use protobuf::ProtobufError;

use protocol_protobuf::request::Request;
use protocol_protobuf::response::Response;

use io2::Parse;

enum Protocol {
    ProtocolBuffer,
    Avro,
}

struct Settings {
    addr: SocketAddr,
    protocol: Protocol,
    db: String,
    config: Option<String>,
    profile: bool,
}

struct Bucket {
    name: String,
    perSecond: u64,
    purpose: Option<String>,
    size: u64,
    until: Option<time::Tm>,
}

impl Parse for Request {
    type Parser = ();
    type Error = io::Error;
    fn parse(_: &mut (),
        buf: &Arc<Vec<u8>>,
        offset: usize)
    -> Option<Result<(Request, usize), io::Error>> {
        match parse_from_bytes::<Request>(&***buf) {
            Ok(m) => {
                let size = m.compute_size() as usize;
                Some(Ok((m, size)))
            },
            Err(e) => match e {
                ProtobufError::WireError(_) => None,
                ProtobufError::MessageNotInitialized { message: _ } => None,
                ProtobufError::IoError(e) => Some(Err(e)),
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("d", "db", "set output file name", "DATABASE");
    opts.optopt("l", "log-level", "Log level [INFO]", "LOG-LEVEL");
    opts.optopt("p", "port", "Port to bind [9231]", "PORT");
    opts.optopt("h", "hostname", "Hostname to bind [0.0.0.0]", "HOSTNAME");
    opts.optopt("c", "config-file", "Configuration file", "CONFIG");
    opts.optflag("", "avro", "Use the avro protocol");
    opts.optflag("", "profile", "");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut builder = env_logger::LogBuilder::new();
    builder.parse(&*matches.opt_str("l").unwrap_or("info".to_owned()));
    builder.init().unwrap();

    let port: u16 = matches.opt_str("p").and_then(|x| x.parse().ok()).unwrap_or(9231);
    let addr = (&*matches.opt_str("h").unwrap_or("0.0.0.0".to_owned()), port).to_socket_addrs().unwrap().next().unwrap();
    let settings = Settings {
        addr: addr.clone(),
        protocol: if matches.opt_present("avro") { Protocol::Avro } else { Protocol::ProtocolBuffer },
        db: matches.opt_str("d").unwrap(),
        config: matches.opt_str("c"),
        profile: matches.opt_present("profile"),
    };
}
