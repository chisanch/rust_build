use bytes::Bytes;
use tokio::net::TcpStream;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    pub async fn new(host: &str, port: u16) -> Result<Client, std::io::Error> {
        let connection = TcpStream::connect((host, port)).await?;
        Ok(Client { connection })
    }

    // pub async fn ping(&mut self, msg: Option<Bytes>) -> Result<Bytes, std::io::Error> {
    //     let mut command = Bytes::from("PING");
    //     if let Some(msg) = msg {
    //         command.extend_from_slice(&b" "[..]);
    //         command.extend_from_slice(&msg[..]);
    //     }
    //     self.connection.write_all(&command[..]).await?;
    //     let mut response = Bytes::new();
    //     self.connection.read_to_end(&mut response).await?;
    //     Ok(response)
    // }
}
