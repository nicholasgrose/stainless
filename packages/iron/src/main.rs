#![forbid(unsafe_code)]

use web::start_server;

mod database;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    start_server("127.0.0.1:8080").await
}
