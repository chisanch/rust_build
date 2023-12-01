use std::time::Duration;

use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
}

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn new(addr: String) -> Result<Client, std::io::Error> {
        let mut parts = addr.splitn(2, ":");
        let host = parts.next().unwrap();
        let port = parts.next().unwrap();
        let stream = TcpStream::connect((host.to_string(), port.parse::<u16>().unwrap())).await?;
        let connection = Connection {
            stream: BufWriter::new(stream),
        };
        Ok(Client { connection })
    }

    pub async fn ping(&mut self) -> Result<Bytes, std::io::Error> {
        let msg = b"*1\r\n$4\r\nPING\r\n";
        self.connection.stream.write_all(msg).await?;
        self.connection.stream.flush().await?;
        let mut buffer = Vec::new();
        self.connection.stream.read_buf(&mut buffer).await?;
        Ok(Bytes::from(buffer))
    }

    pub async fn get(&mut self, key: String) -> Result<Bytes, std::io::Error> {
        let msg = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
        self.connection.stream.write_all(msg.as_bytes()).await?;
        self.connection.stream.flush().await?;
        let mut buffer = Vec::new();
        self.connection.stream.read_buf(&mut buffer).await?;
        Ok(Bytes::from(buffer))
    }

    pub async fn set(
        &mut self,
        key: String,
        value: Bytes,
        expires: Option<Duration>,
    ) -> Result<Bytes, std::io::Error> {
        let mut msg = format!(
            "*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n",
            key.len(),
            key,
            value.len()
        );
        // Append the actual value
        msg.push_str(std::str::from_utf8(&value).unwrap());
        msg.push_str("\r\n");

        // Append expiration if it exists
        if let Some(expires) = expires {
            msg.push_str(&format!("$2\r\nEX\r\n${}\r\n", expires.as_secs()));
        }

        // Write message to stream
        self.connection.stream.write_all(msg.as_bytes()).await?;
        self.connection.stream.flush().await?;

        // Read response
        let mut buffer = Vec::new();
        self.connection.stream.read_buf(&mut buffer).await?;
        Ok(Bytes::from(buffer))
    }
}
