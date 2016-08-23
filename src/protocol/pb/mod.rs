pub mod request;
pub mod response;

use std::io;
use std::sync::Arc;

use futures::{Future, BoxFuture};
use protobuf::CodedInputStream;
use protobuf::Message;
use protobuf::ProtobufError;

use io2::{Parse, Serialize};

use database::{Database, Error};
use handle::{Req, Res, handle, Method, HandlerData};
use protocol::pb::request::{Request, Request_Method};
use protocol::pb::response::{StatusResponse, StatusResponseItem, PongResponse, Response};

impl Req for Request {
    fn method(&self) -> Method {
        match self.get_method() {
            Request_Method::PING => Method::Ping,
            Request_Method::STATUS => Method::Status,
            _ => unimplemented!(),
        }
    }
}

impl Res for Response {
    fn set_pong_response(&mut self) {
        self.set_pongResponse(PongResponse::new())
    }

    fn set_status_response<I: Iterator<Item=(String, i32, i32, i32)>>(&mut self, items: I) {
        let mut status_response = StatusResponse::new();
        status_response.set_items(items.map(|(instance, remaining, reset, limit)| {
            let mut i = StatusResponseItem::new();
            i.set_instance(instance);
            i.set_remaining(remaining);
            i.set_reset(reset);
            i.set_limit(limit);
            i
        }).collect());
        self.set_statusResponse(status_response)
    }
}

pub fn serve_protobuf<D: Database>(r: Request, d: Arc<HandlerData<D>>) -> BoxFuture<Response, Error> {
    let mut response = Response::new();
    response.set_request_id(r.get_id().to_owned());
    handle(&r, response, d).boxed()
}

impl Parse for Request {
    type Parser = ();
    type Error = Error;
    fn parse(_: &mut (),
        buf: &Arc<Vec<u8>>,
        offset: usize)
    -> Option<Result<(Request, usize), Error>> {
        let mut s = CodedInputStream::from_bytes(&buf[offset..]);
        let message = match s.read_message() {
            Ok(m) => m,
            Err(e) => return match e {
                ProtobufError::WireError(_) => None,
                e => Some(Err(Error::IOError(io::Error::new(io::ErrorKind::Other,
                                format!("failed to parse request: {:?}", e))))),
            },
        };
        Some(Ok((message, s.pos() as usize)))
    }
}

impl Serialize for Response {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.write_length_delimited_to_writer(buf).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use protobuf::Message;

    use protocol::pb::request::{Request, Request_Method};
    use io2::Parse;

    fn create_request(id: &str) -> Vec<u8> {
        let mut r = Request::new();
        r.set_id(id.to_owned());
        r.set_field_type("type".to_owned());
        r.set_key("key".to_owned());
        r.set_method(Request_Method::TAKE);
        r.set_count(123);
        r.set_all(true);
        r.write_length_delimited_to_bytes().unwrap()
    }

    #[test]
    fn test_parse_exact() {
        let bytes = create_request("id");
        let len = bytes.len();
        let (req, read) = Request::parse(&mut (), &Arc::new(bytes), 0).unwrap().unwrap();
        assert_eq!(read, len);
        assert_eq!(req.get_id(), "id");
    }

    #[test]
    fn test_parse_incomplete() {
        let mut bytes = create_request("id");
        bytes.pop();
        assert!(Request::parse(&mut (), &Arc::new(bytes), 0).is_none());
    }

    #[test]
    fn test_parse_two_messages() {
        let mut bytes = create_request("id");
        bytes.append(&mut create_request("other_id"));
        let data = Arc::new(bytes);
        let (req, read) = Request::parse(&mut (), &data, 0).unwrap().unwrap();
        assert_eq!("id", req.get_id());
        let req = Request::parse(&mut (), &data, read).unwrap().unwrap().0;
        assert_eq!("other_id", req.get_id());
    }

    #[test]
    fn test_parse_offset() {
        let mut bytes = create_request("id");
        let offset = bytes.len();
        bytes.append(&mut create_request("other_id"));
        let len = bytes.len();
        let (req, read) = Request::parse(&mut (), &Arc::new(bytes), offset).unwrap().unwrap();
        assert_eq!(offset + read, len);
        assert_eq!(req.get_id(), "other_id");
    }
}
