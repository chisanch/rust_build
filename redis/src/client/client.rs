use bytes::{BytesMut, Bytes};
use tokio::{net::TcpStream, io::{AsyncWriteExt, BufWriter}};

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

    pub async fn ping(&mut self, msg: Option<Bytes>) -> Result<Bytes, std::io::Error> {
        
        let message = match msg {
            Some(m) => m,
            None => Bytes::from("PING"),
        };
        self.connection.stream.write_all(&message).await?;
        Ok(message)
    }
    
}
