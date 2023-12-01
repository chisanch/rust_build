extern crate resp;
use resp::{encode, Decoder, Value};
use std::io::BufReader;

pub fn handle_request(buf: Vec<u8>) -> Vec<u8> {
    let buf = BufReader::new(&buf[..]);
    let mut decoder = Decoder::new(buf);
    let value = decoder.decode().unwrap();
    println!("Received Command: {:?}", value);

    match value {
        Value::Array(mut values) => {
            if let Value::Bulk(command) = values.remove(0) {
                match command.as_bytes() {
                    b"PING" => ping(),     // Pass additional arguments if any
                    _ => unimplemented!(), // Other commands can be implemented here
                }
            } else {
                unimplemented!() // Handle other value types if needed
            }
        }
        _ => unimplemented!(), // Handle other top-level value types if needed
    }
}

fn ping() -> Vec<u8> {
    let response = Value::Array(vec![Value::String("PONG".to_string())]);
    encode(&response)
}
