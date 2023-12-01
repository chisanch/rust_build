use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::cmd::handle_request;

pub struct CacheValue {
    pub value: String,
    pub expires_at: Option<SystemTime>,
}

impl CacheValue {
    pub fn from(value: String) -> Self {
        Self {
            value,
            expires_at: None,
        }
    }
}

pub type Cache = Arc<Mutex<HashMap<String, CacheValue>>>;

pub async fn handle_connections(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => return,
            Ok(_) => {
                let response = handle_request(buf.to_vec());
                let _ = stream.write_all(response.as_slice()).await;
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}

pub async fn run() {
    let listener = TcpListener::bind(("127.0.0.1", 6379)).await.unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connections(stream));
            }
            Err(e) => {
                eprintln!("failed to accept connection: {}", e);
            }
        }
    }
}
