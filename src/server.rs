// This file comes mostly from https://github.com/alexcrichton/futures-rs/blob/71801dcd0aaf5049ab901127a01e8a1d869bdf10/futures-minihttp/src/lib.rs
// It has some modifications to make it work with a protocol that's not http(s?)
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;

use net2;
use futures::{self, Future};
use futures::stream::Stream;
use futures_io::{IoFuture, TaskIo};
use futures_mio::{Loop, LoopHandle, TcpStream, TcpListener};

pub use io2::{Parse, Serialize};
use io2::{ParseStream, StreamWriter};

pub trait Service<Req, Resp>: Send + Sync + 'static
    where Req: Send + 'static,
          Resp: Send + 'static
{
    type Fut: Future<Item = Resp> + Send;

    fn process(&self, req: Req) -> Self::Fut;
}

impl<Req, Resp, Fut, F> Service<Req, Resp> for F
    where F: Fn(Req) -> Fut + Send + Sync + 'static,
          Fut: Future<Item = Resp> + Send,
          Req: Send + 'static,
          Resp: Send + 'static
{
    type Fut = Fut;

    fn process(&self, req: Req) -> Fut {
        (self)(req)
    }
}

pub struct Server {
    addr: SocketAddr,
    workers: u32,
}

struct ServerData<S> {
    service: S,
}

impl Server {
    pub fn new(addr: &SocketAddr) -> Server {
        Server {
            addr: *addr,
            workers: 1,
        }
    }

    pub fn workers(&mut self, workers: u32) -> &mut Server {
        if cfg!(unix) {
            self.workers = workers;
        }
        self
    }

    pub fn serve<Req, Resp, S>(&mut self, s: S) -> io::Result<()>
        where Req: Parse,
              Resp: Serialize,
              S: Service<Req, Resp>,
              <S::Fut as Future>::Error: From<Req::Error> + From<io::Error>, // TODO: simplify this?
    {
        let data = Arc::new(ServerData {
            service: s,
        });

        let threads = (0..self.workers - 1).map(|i| {
            let (addr, workers) = (self.addr, self.workers);
            let data = data.clone();
            thread::Builder::new().name(format!("worker{}", i)).spawn(move || {
                let mut lp = Loop::new().unwrap();
                let listener = listener(&addr, workers, lp.handle());
                lp.run(listener.and_then(move |l| {
                    l.incoming().for_each(move |(stream, _)| {
                        handle(stream, data.clone());
                        Ok(()) // TODO: error handling
                    })
                }))
            }).unwrap()
        }).collect::<Vec<_>>();

        let mut lp = Loop::new().unwrap();
        let listener = listener(&self.addr, self.workers, lp.handle());
        lp.run(listener.and_then(move |l| {
            l.incoming().for_each(move |(stream, _)| {
                handle(stream, data.clone());
                Ok(()) // TODO: error handling
            })
        })).unwrap();

        for thread in threads {
            thread.join().unwrap().unwrap();
        }

        Ok(())
    }
}

fn listener(addr: &SocketAddr,
            workers: u32,
            handle: LoopHandle) -> IoFuture<TcpListener> {
    let listener = (|| {
        let listener = try!(net2::TcpBuilder::new_v4());
        try!(configure_tcp(workers, &listener));
        try!(listener.reuse_address(true));
        try!(listener.bind(addr));
        listener.listen(1024)
    })();

    match listener {
        Ok(l) => TcpListener::from_listener(l, addr, handle),
        Err(e) => futures::failed(e).boxed()
    }
}

#[cfg(unix)]
fn configure_tcp(workers: u32, tcp: &net2::TcpBuilder) -> io::Result<()> {
    use net2::unix::*;

    if workers > 1 {
        try!(tcp.reuse_port(true));
    }

    Ok(())
}

#[cfg(windows)]
fn configure_tcp(workers: u32, _tcp: &net2::TcpBuilder) -> io::Result<()> {
    Ok(())
}

trait IoStream: Read + Write + 'static {}

impl<T: ?Sized> IoStream for T
    where T: Read + Write + 'static
{}

fn handle<Req, Resp, S>(stream: TcpStream, data: Arc<ServerData<S>>)
    where Req: Parse,
          Resp: Serialize,
          S: Service<Req, Resp>,
          <S::Fut as Future>::Error: From<Req::Error> + From<io::Error>,
{
    let io = TaskIo::new(stream).map_err(From::from).and_then(|io| {
        let (reader, writer) = io.split();

        let input = ParseStream::new(reader).map_err(From::from);
        let responses = input.and_then(move |req| data.service.process(req));
        StreamWriter::new(writer, responses)
    });
    io.forget();
}
