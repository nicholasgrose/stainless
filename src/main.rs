use std::process::Output;

use anyhow::Error;
use async_trait::async_trait;
use reqwest::Client;
use tokio;

use crate::papermc::{PaperMCProject, PaperMCServer, PaperMCServerApp};

mod papermc;

type Result<T> = std::result::Result<T, Error>;

static SERVER_INFO_DIR_PATH: &str = "stainless/.clients";

pub struct StainlessConfig {
    pub server: ServerType,
}

pub enum ServerType {
    PaperMC(PaperMCServer),
}

pub trait Server<S: Server<S, A>, A: ServerApplication<S, A>> {
    fn server_name(&self) -> &str;
    fn jvm_arguments(&self) -> &Vec<String>;
    fn load_saved_client(&self) -> Result<A>;
    fn client_info_file_path(&self) -> String;
    fn default_version_check_client(&self) -> A;
}

#[async_trait]
pub trait ServerApplication<C: Server<C, A>, A: ServerApplication<C, A>> {
    fn application_name(&self) -> &str;
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

        let server = match stainless_config.server {
            ServerType::PaperMC(config) => config,
        };

        run_server(&server, &http_client)
            .await;

        if server_should_stop() {
            break;
        }
    }
}

async fn run_server<S: Server<S, A>, A: ServerApplication<S, A>>(server: &S, http_client: &Client) {
    let existing_client = match server.load_saved_client() {
        Ok(client_found) => Some(client_found),
        Err(e) => {
            println!("{} Could not load saved server: {}", emoji::symbols::warning::WARNING.glyph, e);
            println!("{} Assuming no server application exists...", emoji::symbols::alphanum::INFORMATION.glyph);
            None
        },
    };

    let default_client = server.default_version_check_client();
    let checking_client = match &existing_client {
        Some(client) => client,
        None => &default_client,
    };

    let existing_client = match checking_client
        .check_for_updated_server(server, http_client)
        .await {
        Ok(update_result) => {
            match update_result {
                Some(updated_client) => {
                    match updated_client.download_server(http_client).await {
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
            println!("{} Attempting to rollback to existing server!", emoji::symbols::alphanum::INFORMATION.glyph);
            existing_client
        }
    };

    let result = match &existing_client {
        Some(server_app) => {
            println!("{} Using server {}!", emoji::symbols::other_symbol::CHECK_MARK.glyph, server_app.application_name());
            server_app.start_server(server)
        }
        None => {
            println!("{} No valid server could be acquired to run!", emoji::symbols::other_symbol::CROSS_MARK.glyph);
            Err(Error::msg("could not find valid server to run"))
        }
    };

    display_server_result(&result);

    if let Some(client) = existing_client {
        match client.save_server_info(&server) {
            Ok(_) => println!("{} Successfully saved server info!", emoji::symbols::other_symbol::CHECK_MARK.glyph),
            Err(e) => println!("{} Unable to save server info: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
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
