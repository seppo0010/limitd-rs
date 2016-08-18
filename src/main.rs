extern crate getopts;
extern crate log;
extern crate protobuf;
extern crate time;

mod protocol_protobuf;

use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::env;

use getopts::Options;
use log::LogLevel;
use protocol_protobuf::request::Request;
use protocol_protobuf::response::Response;

enum Protocol {
    ProtocolBuffer,
    Avro,
}

struct Settings {
    addr: SocketAddr,
    log_level: LogLevel,
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

    let port: u16 = matches.opt_str("p").and_then(|x| x.parse().ok()).unwrap_or(9231);
    let settings = Settings {
        addr: (&*matches.opt_str("h").unwrap_or("0.0.0.0".to_owned()), port).to_socket_addrs().unwrap().next().unwrap(),
        protocol: if matches.opt_present("avro") { Protocol::Avro } else { Protocol::ProtocolBuffer },
        db: matches.opt_str("d").unwrap(),
        config: matches.opt_str("c"),
        profile: matches.opt_present("profile"),
        log_level: match &*matches.opt_str("l").unwrap_or("info".to_owned()) {
            "fatal" => LogLevel::Error,
            "error" => LogLevel::Error,
            "warn" => LogLevel::Warn,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Info,
        }
    };
}
