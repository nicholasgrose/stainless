use std::process::Output;

use anyhow::Error;
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use tokio;

use crate::papermc::{load_saved_client, PaperMCClient, PaperMCConfig, PaperMCProject};

mod papermc;

static CLIENT_INFO_DIR_PATH: &str = "stainless/.clients";

type Result<T> = std::result::Result<T, Error>;

pub trait ClientConfig {
    fn client_name(&self) -> &str;
}

#[async_trait]
pub trait Client<C: ClientConfig, T: Client<C, T>> {
    async fn check_for_updated_client(&self, config: &C, http_client: &HttpClient) -> Result<Option<T>>;
    async fn download_client(&self, http_client: &HttpClient) -> Result<()>;
    fn delete_client(&self) -> Result<()>;
    fn save_client_info(&self, client_config: &C) -> Result<()>;
    fn delete_client_info(&self, client_config: &C) -> Result<()>;
    fn start_client(&self, config: &C) -> Result<Output>;
}

pub struct ServerConfig {
    pub papermc_config: PaperMCConfig,
}

#[tokio::main]
async fn main() {
    if let Err(e) = std::fs::create_dir_all(CLIENT_INFO_DIR_PATH) {
        println!("{} Could not create stainless directories: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
    }

    let http_client = HttpClient::new();

    loop {
        println!("{} Starting server initialization...", emoji::symbols::alphanum::INFORMATION.glyph);

        let server_config = match load_server_configuration() {
            Ok(config) => {
                println!("{} Server configuration loaded!", emoji::symbols::other_symbol::CHECK_MARK.glyph);
                config
            }
            Err(e) => {
                println!("{} Error occurred while loading server configuration: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                break;
            }
        };

        let existing_client = match load_saved_client(&server_config.papermc_config) {
            Ok(client_found) => Some(client_found),
            Err(e) => {
                println!("{} Could not load saved client: {}", emoji::symbols::warning::WARNING.glyph, e);
                println!("{} Assuming no client exists...", emoji::symbols::alphanum::INFORMATION.glyph);
                None
            },
        };

        let default_client = PaperMCClient::default(&server_config.papermc_config.project);
        let checking_client = match &existing_client {
            Some(client) => client,
            None => &default_client
        };

        let existing_client = match checking_client
            .check_for_updated_client(&server_config.papermc_config, &http_client)
            .await {
            Ok(update_result) => {
                match update_result {
                    Some(updated_client) => {
                        match updated_client.download_client(&http_client).await {
                            Ok(_) => Some(updated_client),
                            Err(e) => {
                                println!("{} Failed to download updated client: {}", emoji::symbols::warning::WARNING.glyph, e);
                                existing_client
                            },
                        }
                    },
                    None => existing_client,
                }
            },
            Err(e) => {
                println!("{} Error occurred while checking for updated client: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                existing_client
            },
        };

        let run_result = match &existing_client {
            Some(client) => {
                println!("{} Using client {}!", emoji::symbols::other_symbol::CHECK_MARK.glyph, client.application_download.name);
                client.start_client(&server_config.papermc_config)
            }
            None => {
                println!("{} No client could be acquired to run!", emoji::symbols::other_symbol::CROSS_MARK.glyph);
                break;
            },
        };

        display_server_result(&run_result);

        if server_should_stop() {
            if let Some(client) = existing_client {
                match client.save_client_info(&server_config.papermc_config) {
                    Ok(_) => println!("{} Successfully saved client info!", emoji::symbols::other_symbol::CHECK_MARK.glyph),
                    Err(e) => println!("{} Unable to save client info: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
                }
            }

            break;
        }
    }
}

fn display_server_result(run_result: &Result<Output>) {
    match run_result {
        Ok(result) => {
            println!("{} Server exited with: ({})", emoji::symbols::alphanum::INFORMATION.glyph, result.status)
        }
        Err(e) => {
            println!("{} Server encountered an error: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
        }
    }
}

// TODO: Make this load from a config file
fn load_server_configuration() -> Result<ServerConfig> {
    println!("{} Loading server configuration...", emoji::symbols::alphanum::INFORMATION.glyph);

    Ok(ServerConfig {
        papermc_config: PaperMCConfig {
            client_name: "papermc".to_string(),
            project: PaperMCProject {
                name: String::from("paper"),
                version: String::from("1.18.1"),
            },
            java_arguments: vec!(),
        },
    })
}

// TODO: Make this take user input
fn server_should_stop() -> bool {
    true
}
