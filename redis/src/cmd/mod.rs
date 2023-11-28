mod get;
pub use get::Get;

mod ping;
pub use ping::Ping;

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Ping(Ping)
}

impl Command {
    
}