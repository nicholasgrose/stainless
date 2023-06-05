#![forbid(unsafe_code)]
#![feature(error_generic_member_access, provide_any)]

use error::IronError;
use web::start_server;

mod database;
mod error;
mod shared;
mod web;

type Result<T> = anyhow::Result<T, IronError>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    start_server("127.0.0.1:8080")?.await?
}
