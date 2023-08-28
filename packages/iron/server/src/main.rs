#![forbid(unsafe_code)]

use crate::config::IronConfig;
use web::start_server;

mod config;
mod database;
mod manager;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = IronConfig::load().unwrap_or_default();
    tracing::info!("{:?}", config);

    start_server(config).await
}
