use bytes::Bytes;
use clap::{Parser, Subcommand};
use redis::client::Client;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(name = "redis-cli", version, about = "Issue Redis commands")]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = 6379)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {},
    Get {
        key: String,
    },
    Set {
        key: String,

        #[clap(value_parser = bytes_from_str)]
        value: Bytes,

        #[clap(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Result<Bytes, Infallible> {
    Ok(Bytes::from(src.to_string()))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.host, cli.port);
    let mut _client = Client::new(addr).await.expect("Failed to create client");

    match cli.command {
        Command::Ping {} => {
            let response = _client.ping().await;
            println!("Response: {:?}", response);
        }
        Command::Get { key } => {
            let response = _client.get(key).await;
            println!("Response: {:?}", response);
        }
        Command::Set {
            key,
            value,
            expires,
        } => {
            let response = _client.set(key, value, expires).await;
            println!("Response: {:?}", response);
        }
        _ => {
            println!("Invalid command");
        }
    }
}
