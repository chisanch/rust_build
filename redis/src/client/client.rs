use bytes::Bytes;
use tokio::net::TcpStream;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    pub async fn new(addr: String) -> Result<Client, std::io::Error> {
        let mut parts = addr.splitn(2, ":");
        let host = parts.next().unwrap();
        let port = parts.next().unwrap();    
        let connection = TcpStream::connect((host.to_string(), port.parse::<u16>().unwrap())).await?;
        Ok(Client { connection })
    }
}
