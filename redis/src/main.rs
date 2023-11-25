use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connections(stream: &mut TcpStream) -> () {
    loop {
        // Read
        let mut buf: [u8; 512] = [0; 512];
        let num_bytes_read = stream.read(&mut buf).unwrap();
        if num_bytes_read == 0 {
            return;
        }
        // Write
        let res = "+PONG\r\n";
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap_or_else(|e| {
        panic!("failed to bind to socket: {}", e);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(mut r_stream) => {
                handle_connections(&mut r_stream);
            }
            Err(e) => {
                panic!("failed to accept connection: {}", e);
            }
        }
    }
}
