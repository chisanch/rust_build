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

    pub async fn ping(&mut self) -> Result<Bytes, std::io::Error> {
        let msg = b"*1\r\n$4\r\nPING\r\n";
        self.connection.stream.write_all(msg).await?;
        self.connection.stream.flush().await?;
        let mut buffer = Vec::new();
        self.connection.stream.read_buf(&mut buffer).await?;
        Ok(Bytes::from(buffer))
    }
}
