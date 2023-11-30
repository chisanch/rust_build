use bytes::Bytes;
use resp::{encode, Value};

#[derive(Debug)]
pub struct Ping {
    msg: Option<Bytes>,
}

impl Ping {
    pub fn new(msg: Option<Bytes>) -> Ping {
        Ping { msg }
    }

    pub fn apply_encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match &self.msg {
            Some(msg) => {
                bytes.extend_from_slice(b"+");
                bytes.extend_from_slice(msg);
                bytes.extend_from_slice(b"\r\n");
            }
            None => {
                bytes.extend_from_slice(b"+PONG\r\n");
            }
        }
        bytes
    }
}
