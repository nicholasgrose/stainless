#![forbid(unsafe_code)]

use crate::web::IronGrpcService;

macro_rules! required_def {
    ($message:expr) => {
        $message
            .as_ref()
            .with_context(|| format!("required definition not provided: {}", stringify!($message)))
    };
}

mod database;
mod manager;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    IronGrpcService::default().start().await
}
