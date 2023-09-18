#![forbid(unsafe_code)]

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::web::IronGrpcService;

mod database;
mod manager;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_logging();
    IronGrpcService::new().start().await
}

fn start_logging() {
    let console_layer = console_subscriber::spawn();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(console_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
