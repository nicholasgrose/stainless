use anyhow::Error;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCProject, PaperMCServer, PaperMCServerApp};
use crate::server::initialize_server_loop;

mod papermc;
mod server;
mod config;

type Result<T> = std::result::Result<T, Error>;

static SERVER_INFO_DIR_PATH: &str = "stainless/.clients";

#[tokio::main]
async fn main() {
    if let Err(e) = std::fs::create_dir_all(SERVER_INFO_DIR_PATH) {
        println!("{} Could not create stainless directories: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
        return;
    }

    let stainless_config = match config::load_stainless_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{} Error occurred while loading Stainless configuration: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
            return;
        }
    };

    let http_client = Client::new();

    initialize_server_loop(&stainless_config.server, &http_client).await
}
