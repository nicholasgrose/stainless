use anyhow::Error;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCClient, PaperMCProject};

mod papermc;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    loop {
        let server_config = match load_server_configuration(&http_client).await {
            Ok(config) => config,
            Err(_) => break
        };

        match start_server(&server_config, &http_client).await {
            Ok(_) => {},
            Err(e) => eprintln!("Error occurred while running server: {}", e)
        };

        if server_should_stop() {
            match server_config.client.delete_client(&server_config.client_dir_path) {
                Ok(_) => {}
                Err(e) => eprintln!("Error occurred while deleting client: {}", e)
            };
            break;
        }
    }
}

// TODO: Make this actually download and start a java server
async fn start_server(server_config: &ServerConfig, http_client: &Client) -> Result<(), Error> {
    server_config.client.download_client(&server_config.client_dir_path, http_client).await
}

struct ServerConfig {
    client: PaperMCClient,
    client_dir_path: String,
    java_arguments: Vec<String>,
}

// TODO: Make this load from a config file
async fn load_server_configuration(http_client: &Client) -> Result<ServerConfig, Error> {
    let project = PaperMCProject {
        name: String::from("paper"),
        version: String::from("1.18"),
    };
    let latest_client = papermc::query::latest_papermc_client_for_project(project, http_client)
        .await;

    Ok(ServerConfig {
        client: latest_client?,
        client_dir_path: String::from("."),
        java_arguments: vec!(),
    })
}

// TODO: Make this take user input
fn server_should_stop() -> bool {
    true
}
