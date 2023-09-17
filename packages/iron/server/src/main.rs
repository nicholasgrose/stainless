#![forbid(unsafe_code)]

use crate::web::IronGrpcService;

mod database;
mod manager;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_logging();
    IronGrpcService::default().start().await
}

fn start_logging() {
    console_subscriber::init();
}
