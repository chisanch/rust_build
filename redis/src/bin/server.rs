use redis::server;

#[tokio::main]
async fn main() {
    server::run().await;
}
