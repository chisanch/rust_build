use crate::cmd::Ping;
use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
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
            buffer: BytesMut::new(),
        };
        Ok(Client { connection })
    }

    // Sends a PING command to the server and returns the response.
    // If a message is provided, the server will return it in the response.
    // Takes in an optional message to send to the server.
    // Takes in message as bytes, apply_encode() will convert to Vec<u8>.
    // Returns a Result with the response from the server.
    pub async fn ping(&mut self, msg: Option<Bytes>) -> Result<Bytes, std::io::Error> {
        let ping = Ping::new(msg);
        let msg = ping.apply_encode();
        self.connection.stream.write_all(&msg).await?;
        self.connection.stream.flush().await?;
        let mut buffer = Vec::new();
        self.connection.stream.read_buf(&mut buffer).await?;
        Ok(Bytes::from(buffer))
    }
}
