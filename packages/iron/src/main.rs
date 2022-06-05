#![feature(backtrace)]

use error::IronError;
use web::start_server;

mod database;
mod error;
mod shared;
mod web;

type Result<T> = anyhow::Result<T, IronError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server("0.0.0.0:8080")?.await
}
