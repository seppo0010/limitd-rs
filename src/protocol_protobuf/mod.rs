pub mod request;
pub mod response;

use std::io;
use std::sync::Arc;

use protobuf::parse_from_bytes;
use protobuf::Message;
use protobuf::ProtobufError;

use io2::Parse;

use protocol_protobuf::request::Request;
use protocol_protobuf::response::Response;

impl Parse for Request {
    type Parser = ();
    type Error = io::Error;
    fn parse(_: &mut (),
        buf: &Arc<Vec<u8>>,
        offset: usize)
    -> Option<Result<(Request, usize), io::Error>> {
        match parse_from_bytes::<Request>(&buf[offset..]) {
            Ok(m) => {
                let size = m.compute_size() as usize;
                Some(Ok((m, size)))
            },
            Err(e) => match e {
                ProtobufError::IoError(e) => None,
                e => Some(Err(io::Error::new(io::ErrorKind::Other,
                                format!("failed to parse request: {:?}", e)))),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use protobuf::Message;

    use protocol_protobuf::request::{Request, Request_Method};
    use io2::Parse;

    fn create_request(id: &str) -> Vec<u8> {
        let mut r = Request::new();
        r.set_id(id.to_owned());
        r.set_field_type("type".to_owned());
        r.set_key("key".to_owned());
        r.set_method(Request_Method::TAKE);
        r.set_count(123);
        r.set_all(true);
        r.write_to_bytes().unwrap()
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
