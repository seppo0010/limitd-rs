extern crate env_logger;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate futures_io;
extern crate futures_mio;
extern crate getopts;
#[macro_use]
extern crate log;
extern crate net2;
extern crate protobuf;
extern crate time;

mod protocol_protobuf;
mod io2;
mod server;

use std::env;
use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

use futures::{Finished, finished};
use getopts::Options;

pub use io2::{Parse, Serialize};
use protocol_protobuf::request::Request;
use protocol_protobuf::response::Response;
use server::Server;

enum Protocol {
    ProtocolBuffer,
    Avro,
}

fn serve_protobuf(_r: Request) -> Finished<Response, io::Error> {
    finished(Response::new())
}

impl Protocol {
    pub fn serve(&self, server: &mut Server) -> io::Result<()> {
        match *self {
            Protocol::ProtocolBuffer => server.serve(serve_protobuf),
            _ => unimplemented!(),
        }
    }
}

struct Settings {
    addr: SocketAddr,
    protocol: Protocol,
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
        addr: addr,
        protocol: if matches.opt_present("avro") { Protocol::Avro } else { Protocol::ProtocolBuffer },
    };

    let mut server = Server::new(&settings.addr);
    server.workers(1);
    settings.protocol.serve(&mut server).unwrap();
}
