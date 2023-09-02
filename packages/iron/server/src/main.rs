#![forbid(unsafe_code)]

use crate::web::IronGrpcService;

mod database;
mod manager;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    IronGrpcService::default().start().await
}
