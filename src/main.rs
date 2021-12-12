use anyhow::Error;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCClient, PaperMCProject};

mod papermc;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    loop {
        println!("Starting server initialization...");

        let server_config = match load_server_configuration(&http_client).await {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error occurred while loading server configuration: {}", e);
                break;
            }
        };

        let run_client = match start_server(&server_config, &http_client).await {
            Ok(run_client) => Some(run_client),
            Err(e) => {
                eprintln!("Error occurred while running server: {}", e);
                None
            },
        };

        if server_should_stop() {
            if let Some(client) = run_client {
                match client.delete_client(&server_config.client_dir_path) {
                    Ok(_) => println!("Client deleted successfully!"),
                    Err(e) => eprintln!("Error occurred while deleting client: {}", e),
                };
            }
            break;
        }
    }
}

// TODO: Make this actually download and start a java server
async fn start_server<'a>(server_config: &'a ServerConfig, http_client: &Client) -> Result<&'a PaperMCClient, Error> {
    let current_client = match &server_config.latest_client {
        Some(latest_client) => {
            match latest_client.download_client(&server_config.client_dir_path, http_client).await {
                Ok(()) => Some(latest_client),
                Err(e) => {
                    println!("{} Failed to download latest client! ({})", emoji::symbols::warning::WARNING.glyph, e);
                    println!("{} Attempting to roll back to previous client.", emoji::symbols::alphanum::INFORMATION.glyph);
                    Option::from(&server_config.previous_client)
                }
            }
        },
        None => {
            println!("{} No new client could be found.", emoji::symbols::alphanum::INFORMATION.glyph);
            println!("{} Attempting to roll back to previous client.", emoji::symbols::alphanum::INFORMATION.glyph);
            Option::from(&server_config.previous_client)
        },
    };

    match current_client {
        Some(client) => {
            client.start_server();
            Ok(client)
        },
        None => Err(Error::msg("no valid server client could be acquired"))
    }
}

#[allow(dead_code)]
struct ServerConfig {
    previous_client: Option<PaperMCClient>,
    latest_client: Option<PaperMCClient>,
    client_dir_path: String,
    java_arguments: Vec<String>,
}

// TODO: Make this load from a config file
async fn load_server_configuration(http_client: &Client) -> Result<ServerConfig, Error> {
    println!("Loading server configuration...");
    let project = PaperMCProject {
        name: String::from("paper"),
        version: String::from("1.18"),
    };
    let latest_client = papermc::query::latest_papermc_client_for_project(project, http_client)
        .await?;

    println!("{} Server configuration loaded!", emoji::symbols::other_symbol::CHECK_MARK.glyph);

    Ok(ServerConfig {
        previous_client: None,
        latest_client: Some(latest_client),
        client_dir_path: String::from("."),
        java_arguments: vec!(),
    })
}

// TODO: Make this take user input
fn server_should_stop() -> bool {
    true
}
