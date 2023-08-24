#![forbid(unsafe_code)]

use crate::config::IronConfig;
use web::start_server;

mod config;
mod database;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    start_server(IronConfig::load().unwrap_or_default()).await
}
