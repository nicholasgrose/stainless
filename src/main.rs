use std::process::Output;

use anyhow::Error;
use async_trait::async_trait;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCConfig, PaperMCProject, PaperMCServer};

mod papermc;

type Result<T> = std::result::Result<T, Error>;

static SERVER_INFO_DIR_PATH: &str = "stainless/.clients";

pub struct StainlessConfig {
    pub server: Server,
}

pub enum Server {
    PaperMC(PaperMCConfig),
}

pub trait ServerConfig<C: ServerConfig<C, A>, A: ServerApplication<C, A>> {
    fn server_name(&self) -> &str;
    fn jvm_arguments(&self) -> &Vec<String>;
    fn load_saved_client(&self) -> Result<A>;
    fn client_info_file_path(&self) -> String;
}

#[async_trait]
pub trait ServerApplication<C: ServerConfig<C, A>, A: ServerApplication<C, A>> {
    async fn check_for_updated_server(&self, config: &C, http_client: &Client) -> Result<Option<A>>;
    async fn download_server(&self, http_client: &Client) -> Result<()>;
    fn delete_server(&self) -> Result<()>;
    fn save_server_info(&self, client_config: &C) -> Result<()>;
    fn delete_server_info(&self, client_config: &C) -> Result<()>;
    fn start_server(&self, config: &C) -> Result<Output>;
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
            Ok(config) => {
                println!("{} Stainless configuration loaded!", emoji::symbols::other_symbol::CHECK_MARK.glyph);
                config
            }
            Err(e) => {
                println!("{} Error occurred while loading Stainless configuration: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                break;
            }
        };

        let config = match stainless_config.server {
            Server::PaperMC(config) => config,
        };

        let existing_client = match config.load_saved_client() {
            Ok(client_found) => Some(client_found),
            Err(e) => {
                println!("{} Could not load saved server: {}", emoji::symbols::warning::WARNING.glyph, e);
                println!("{} Assuming no server application exists...", emoji::symbols::alphanum::INFORMATION.glyph);
                None
            },
        };

        let default_client = PaperMCServer::default(&config);
        let checking_client = match &existing_client {
            Some(client) => client,
            None => &default_client
        };

        let existing_client = match checking_client
            .check_for_updated_server(&config, &http_client)
            .await {
            Ok(update_result) => {
                match update_result {
                    Some(updated_client) => {
                        match updated_client.download_server(&http_client).await {
                            Ok(_) => Some(updated_client),
                            Err(e) => {
                                println!("{} Failed to download updated server: {}", emoji::symbols::warning::WARNING.glyph, e);
                                existing_client
                            }
                        }
                    }
                    None => existing_client,
                }
            }
            Err(e) => {
                println!("{} Error occurred while checking for updated server: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                existing_client
            }
        };

        let run_result = match &existing_client {
            Some(server) => {
                println!("{} Using server {}!", emoji::symbols::other_symbol::CHECK_MARK.glyph, server.application_download.name);
                server.start_server(&config)
            }
            None => {
                println!("{} No server could be acquired to run!", emoji::symbols::other_symbol::CROSS_MARK.glyph);
                break;
            }
        };

        display_server_result(&run_result);

        if server_should_stop() {
            if let Some(client) = existing_client {
                match client.save_server_info(&config) {
                    Ok(_) => println!("{} Successfully saved server info!", emoji::symbols::other_symbol::CHECK_MARK.glyph),
                    Err(e) => println!("{} Unable to save server info: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
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
fn load_server_configuration() -> Result<StainlessConfig> {
    println!("{} Loading server configuration...", emoji::symbols::alphanum::INFORMATION.glyph);

    Ok(StainlessConfig {
        server: Server::PaperMC(PaperMCConfig {
            server_name: "papermc".to_string(),
            project: PaperMCProject {
                name: String::from("paper"),
                version: String::from("1.18.1"),
            },
            jvm_arguments: vec!(),
        })
    })
}

// TODO: Make this take user input
fn server_should_stop() -> bool {
    true
}
