#[macro_use]
extern crate lazy_static;

pub mod client;
pub use client::Client;

pub mod server;
pub use server::{run, Cache};

pub mod cmd;
pub use cmd::handle_request;
