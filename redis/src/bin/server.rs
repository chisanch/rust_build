extern crate resp;
#[allow(unused_imports)]
use resp::{encode, encode_slice, Decoder, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connections(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => return,
            Ok(bytes_read) => {
                println!("bytes read: {}", bytes_read);
                println!("buf: {:?}", &buf[..bytes_read]);

                if let Err(e) = stream.write(b"+PONG\r\n").await {
                    eprintln!("Failed to write to stream: {}", e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .unwrap_or_else(|e| {
            panic!("failed to bind to socket: {}", e);
        });

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
