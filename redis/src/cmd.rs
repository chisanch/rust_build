extern crate resp;
use crate::server::{Cache, CacheValue};
use resp::{encode, Decoder, Value};
use std::{
    collections::HashMap,
    io::BufReader,
    sync::{Arc, Mutex},
};

lazy_static! {
    pub static ref CACHE: Cache = Arc::new(Mutex::new(HashMap::new()));
}

pub fn handle_request(buf: Vec<u8>) -> Vec<u8> {
    let buf = BufReader::new(&buf[..]);
    let mut decoder = Decoder::new(buf);
    let value = decoder.decode().unwrap();
    println!("Received Command: {:?}", value);

    match value {
        Value::Array(mut values) => {
            if let Value::Bulk(command) = values.remove(0) {
                match command.as_bytes() {
                    b"PING" => ping(),
                    b"GET" => get(values),
                    b"SET" => set(values),
                    _ => unimplemented!(), // Other commands can be implemented here
                }
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

fn ping() -> Vec<u8> {
    let response = Value::Array(vec![Value::String("PONG".to_string())]);
    encode(&response)
}

fn get(mut args: Vec<Value>) -> Vec<u8> {
    if let Some(Value::Bulk(key)) = args.pop() {
        let kv_store = CACHE.lock().unwrap();
        if let Some(value) = kv_store.get(&key) {
            return encode(&Value::Bulk(value.value.clone()));
        }
        encode(&Value::Bulk("nil".to_string()))
    } else {
        encode(&Value::Error(
            "ERR wrong number of arguments for 'get' command".to_string(),
        ))
    }
}

fn set(mut args: Vec<Value>) -> Vec<u8> {
    if args.len() != 2 {
        return encode(&Value::Error(
            "ERR wrong number of arguments for 'set' command".to_string(),
        ));
    }
    let value = args.pop().unwrap();
    let key = args.pop().unwrap();

    if let (Value::Bulk(key), Value::Bulk(value)) = (key, value) {
        let mut kv_store = CACHE.lock().unwrap();
        kv_store.insert(key, CacheValue::from(value));
        return encode(&Value::String("OK".to_string()));
    }
    encode(&Value::Error("ERR invalid key or value".to_string()))
}
