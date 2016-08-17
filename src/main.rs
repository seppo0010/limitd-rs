extern crate log;
extern crate getopts;

use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::env;

use getopts::Options;

enum Protocol {
    ProtocolBuffer,
    Avro,
}

struct Settings {
    addr: SocketAddr,
    log_level: log::LogLevel,
    protocol: Protocol,
    db: String,
    config: Option<String>,
    profile: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("d", "db", "set output file name", "DATABASE");
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
        log_level: log::LogLevel::Info,
        protocol: if matches.opt_present("avro") { Protocol::Avro } else { Protocol::ProtocolBuffer },
        db: matches.opt_str("d").unwrap(),
        config: matches.opt_str("c"),
        profile: matches.opt_present("profile"),
    };
}
