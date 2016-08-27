extern crate db_key;
extern crate env_logger;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate futures_io;
extern crate futures_mio;
extern crate getopts;
extern crate json;
extern crate leveldb;
#[macro_use]
extern crate log;
extern crate net2;
extern crate protobuf;
extern crate time;
extern crate yaml_rust;

mod bucket;
mod database;
mod handle;
mod io2;
mod protocol;
mod server;

use std::env;
use std::io::Read;
use std::fs::File;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use getopts::Options;

use protocol::Protocol;
use server::Server;

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
    let protocol = if matches.opt_present("avro") { Protocol::Avro } else { Protocol::ProtocolBuffer };
    let db = database::LevelDB::<database::Key>::new(&*matches.opt_str("d").unwrap()).unwrap();
    let config = matches.opt_str("c").and_then(|c| {
        let mut f = match File::open(c) {
            Ok(f) => f,
            Err(_) => return None
        };
        let mut s = String::new();
        f.read_to_string(&mut s).ok().map(|_| s)
    }).unwrap_or("".to_owned());
    let buckets = bucket::Buckets::new(&*config).unwrap();
    let handler_data = handle::HandlerData::new(db, buckets);

    let mut server = Server::new(&addr);
    server.workers(1);
    protocol.serve(&mut server, Arc::new(handler_data)).unwrap();
}
