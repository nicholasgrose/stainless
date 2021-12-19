use anyhow::Error;
use emoji::symbols::other_symbol::CROSS_MARK;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCServer, PaperMCServerApp};
use crate::server::initialize_server_loop;

mod papermc;
mod server;
mod config;

type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() {
    let http_client = Client::new();

    let stainless_config = match config::load_stainless_config(&http_client).await {
        Ok(config) => config,
        Err(e) => {
            println!("{} Error occurred while loading Stainless configuration: {}", CROSS_MARK.glyph, e);
            return;
        }
    };

    initialize_server_loop(&stainless_config.server, &http_client).await
}
