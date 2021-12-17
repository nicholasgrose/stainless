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
    fn load_saved_server_app(&self) -> Result<A>;
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
            Ok(config) => config,
            Err(e) => {
                println!("{} Error occurred while loading Stainless configuration: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
                break;
            }
        };

        let server = match stainless_config.server {
            ServerType::PaperMC(config) => config,
        };

        if let Err(e) = run_server(&server, &http_client).await {
            println!("{} Server encountered unrecoverable error: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
            break;
        }

        if server_should_stop() {
            break;
        }
    }
}

async fn run_server<S: Server<S, A>, A: ServerApplication<S, A>>(server: &S, http_client: &Client) -> Result<()> {
    let server_app = acquire_server_app(server, http_client)
        .await;

    let run_result = match &server_app {
        Some(server_app) => {
            println!("{} Using server {}!", emoji::symbols::other_symbol::CHECK_MARK.glyph, server_app.application_name());
            server_app.start_server(server)
        }
        None => {
            println!("{} No valid server could be acquired to run!", emoji::symbols::other_symbol::CROSS_MARK.glyph);
            return Err(Error::msg("could not find valid server to run"));
        }
    };

    display_server_result(&run_result);
    save_server_info_if_exists(server, &server_app);

    Ok(())
}

async fn acquire_server_app<S: Server<S, A>, A: ServerApplication<S, A>>(server: &S, http_client: &Client) -> Option<A> {
    let existing_server_app = match server.load_saved_server_app() {
        Ok(client_found) => Some(client_found),
        Err(e) => {
            println!("{} Could not load saved server: {}", emoji::symbols::warning::WARNING.glyph, e);
            println!("{} Assuming no server application exists...", emoji::symbols::alphanum::INFORMATION.glyph);
            None
        }
    };

    update_server_app(existing_server_app, server, http_client).await
}

async fn update_server_app<S: Server<S, A>, A: ServerApplication<S, A>>(existing_server_app: Option<A>, server: &S, http_client: &Client) -> Option<A> {
    let default_server_app = server.default_version_check_client();
    let checking_server_app = match &existing_server_app {
        Some(server_app) => server_app,
        None => &default_server_app,
    };

    match checking_server_app
        .check_for_updated_server(server, http_client)
        .await {
        Ok(update_result) => {
            download_server_app_if_new_one_exists(update_result, existing_server_app, http_client)
                .await
        }
        Err(e) => {
            println!("{} Error occurred while checking for updated server: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e);
            println!("{} Attempting to roll back to existing server!", emoji::symbols::alphanum::INFORMATION.glyph);
            existing_server_app
        }
    }
}

async fn download_server_app_if_new_one_exists<S: Server<S, A>, A: ServerApplication<S, A>>(update_result: Option<A>, existing_server_app: Option<A>, http_client: &Client) -> Option<A> {
    match update_result {
        Some(updated_client) => {
            match updated_client.download_server(http_client).await {
                Ok(_) => Some(updated_client),
                Err(e) => {
                    println!("{} Failed to download updated server: {}", emoji::symbols::warning::WARNING.glyph, e);
                    existing_server_app
                }
            }
        }
        None => existing_server_app,
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

fn save_server_info_if_exists<S: Server<S, A>, A: ServerApplication<S, A>>(server: &S, server_app: &Option<A>) {
    if let Some(client) = server_app {
        match client.save_server_info(server) {
            Ok(_) => println!("{} Successfully saved server info!", emoji::symbols::other_symbol::CHECK_MARK.glyph),
            Err(e) => println!("{} Unable to save server info: {}", emoji::symbols::other_symbol::CROSS_MARK.glyph, e)
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
