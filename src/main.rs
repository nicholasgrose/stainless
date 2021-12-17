use anyhow::Error;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCProject, PaperMCServer, PaperMCServerApp};

mod papermc;
mod server;
mod config;

type Result<T> = std::result::Result<T, Error>;

static SERVER_INFO_DIR_PATH: &str = "stainless/.clients";

#[tokio::main]
async fn main() {
    if let Err(e) = std::fs::create_dir_all(SERVER_INFO_DIR_PATH) {
        println!("{} Could not create stainless directories: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
    }

    let http_client = Client::new();

    loop {
        println!("{} Starting server initialization...", emoji::symbols::alphanum::INFORMATION.glyph);

        let stainless_config = match config::load_server_configuration() {
            Ok(config) => config,
            Err(e) => {
                println!("{} Error occurred while loading Stainless configuration: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                break;
            }
        };

        if let Err(e) = server::run_configured_server(&stainless_config.server, &http_client).await {
            println!("{} Server encountered unrecoverable error: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
            break;
        }

        if server_should_stop() {
            break;
        }
    }
}

// TODO: Make this take user input
fn server_should_stop() -> bool {
    true
}
