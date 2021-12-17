use anyhow::Error;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCProject, PaperMCServer, PaperMCServerApp};

mod papermc;
mod server;

type Result<T> = std::result::Result<T, Error>;

static SERVER_INFO_DIR_PATH: &str = "stainless/.clients";

pub struct StainlessConfig {
    pub server: ServerType,
}

pub enum ServerType {
    PaperMC(PaperMCServer),
}

#[tokio::main]
async fn main() {
    if let Err(e) = std::fs::create_dir_all(SERVER_INFO_DIR_PATH) {
        println!("{} Could not create stainless directories: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
    }

    let http_client = Client::new();

    loop {
        println!("{} Starting server initialization...", emoji::symbols::alphanum::INFORMATION.glyph);

        let stainless_config = match load_server_configuration() {
            Ok(config) => config,
            Err(e) => {
                println!("{} Error occurred while loading Stainless configuration: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                break;
            }
        };

        let server = match stainless_config.server {
            ServerType::PaperMC(config) => config,
        };

        if let Err(e) = server::run_server(&server, &http_client).await {
            println!("{} Server encountered unrecoverable error: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
            break;
        }

        if server_should_stop() {
            break;
        }
    }
}

// TODO: Make this load from a config file
fn load_server_configuration() -> Result<StainlessConfig> {
    println!("{} Loading server configuration...", emoji::symbols::alphanum::INFORMATION.glyph);

    println!("{} Stainless configuration loaded!", emoji::symbols::other_symbol::CHECK_MARK.glyph);

    Ok(StainlessConfig {
        server: ServerType::PaperMC(PaperMCServer {
            server_name: "papermc".to_string(),
            project: PaperMCProject {
                name: "paper".to_string(),
                version: "1.18.1".to_string(),
            },
            jvm_arguments: vec!(),
        })
    })
}

// TODO: Make this take user input
fn server_should_stop() -> bool {
    true
}
