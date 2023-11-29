use bytes::Bytes;
use resp::{Decoder, Value};

#[derive(Debug)]
pub struct Ping {
    msg: Option<Bytes>
}

impl Ping {
    pub fn new(msg: Option<Bytes>) -> Ping {
        let value = msg.and_then(|bytes| {
            let mut decoder = Decoder::new(&bytes[..]);
            decoder.decode().ok()
        });

        Ping { msg: value }
    }
}