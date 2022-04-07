use std::process::ExitStatus;

use anyhow::Error;
use async_trait::async_trait;
use emoji::symbols::alphanum::INFORMATION;
use emoji::symbols::other_symbol::{CHECK_MARK, CROSS_MARK};
use reqwest::Client;
use tokio::select;
use tokio::sync::mpsc::Receiver;

use crate::config::ServerType;
use crate::server::control::create_control_socket;

mod control;

pub trait Server<S: Server<S, A>, A: ServerApplication<S, A>> {
    fn server_name(&self) -> &str;
    fn jvm_arguments(&self) -> &Vec<String>;
    fn load_saved_server_app(&self) -> crate::Result<A>;
    fn client_info_file_path(&self) -> String;
    fn default_version_check_client(&self) -> A;
}

#[async_trait]
pub trait ServerApplication<C: Server<C, A>, A: ServerApplication<C, A>> {
    fn application_name(&self) -> &str;
    async fn check_for_updated_server(
        &self,
        config: &C,
        http_client: &Client,
    ) -> crate::Result<Option<A>>;
    async fn download_server(&self, http_client: &Client) -> crate::Result<()>;
    fn delete_server(&self) -> crate::Result<()>;
    fn save_server_info(&self, client_config: &C) -> crate::Result<()>;
    fn delete_server_info(&self, client_config: &C) -> crate::Result<()>;
    async fn start_server(
        &self,
        config: &C,
        input_receiver: &mut Receiver<u8>,
    ) -> crate::Result<ExitStatus>;
}

pub async fn begin_server_task(server_type: &ServerType, http_client: &Client) {
    let control_socket_result = create_control_socket().await;

    match control_socket_result {
        Ok(mut socket) => {
            select! {
                control_thread_result = socket.control_thread => {
                    match control_thread_result {
                        Ok(thread_run_result) => match thread_run_result {
                            Ok(_) => println!("{} Control thread exited without error", INFORMATION.glyph),
                            Err(e) => println!("{} Error encountered while running control: {}", CROSS_MARK.glyph, e),
                        },
                        Err(e) => println!("{} Error encountered while spawning control: {}", CROSS_MARK.glyph, e),
                    }
                }
                _ = initialize_server_loop(server_type, http_client, &mut socket.control_receiver) => {}
            }
        }
        Err(e) => println!("{} Error making control socket: {}", CROSS_MARK.glyph, e),
    }
}

async fn initialize_server_loop(
    server_type: &ServerType,
    http_client: &Client,
    input_receiver: &mut Receiver<u8>,
) {
    println!("{} Entering server loop...", INFORMATION.glyph);

    loop {
        println!("{} Starting server...", INFORMATION.glyph);

        if let Err(e) = run_configured_server(server_type, http_client, input_receiver).await {
            println!(
                "{} Server encountered unrecoverable error: {}",
                CROSS_MARK.glyph, e
            );
            break;
        }

        let should_stop_result = control::server_should_stop(input_receiver).await;

        match should_stop_result {
            Ok(should_stop) => {
                if should_stop {
                    println!("{} Server stopped!", INFORMATION.glyph);

                    break;
                } else {
                    println!("{} Restarting...", INFORMATION.glyph)
                }
            }
            Err(e) => {
                println!(
                    "{} Stainless encountered error reading input: {}",
                    CROSS_MARK.glyph, e
                );

                break;
            }
        }
    }
}

pub async fn run_configured_server(
    server_type: &ServerType,
    http_client: &Client,
    input_receiver: &mut Receiver<u8>,
) -> crate::Result<()> {
    let ServerType::PaperMC(server) = server_type;

    run_server(server, http_client, input_receiver).await
}

async fn run_server<S: Server<S, A>, A: ServerApplication<S, A>>(
    server: &S,
    http_client: &Client,
    input_receiver: &mut Receiver<u8>,
) -> crate::Result<()> {
    let server_app = acquire_server_app(server, http_client).await;

    let run_result = match &server_app {
        Some(server_app) => {
            println!(
                "{} Using server {}!",
                CHECK_MARK.glyph,
                server_app.application_name()
            );
            server_app.start_server(server, input_receiver).await
        }
        None => {
            println!(
                "{} No valid server could be acquired to run!",
                CROSS_MARK.glyph
            );
            return Err(Error::msg("Could not find valid server to run."));
        }
    };

    display_server_result(&run_result);
    save_server_info_if_exists(server, &server_app);

    Ok(())
}

async fn acquire_server_app<S: Server<S, A>, A: ServerApplication<S, A>>(
    server: &S,
    http_client: &Client,
) -> Option<A> {
    let existing_server_app = match server.load_saved_server_app() {
        Ok(client_found) => Some(client_found),
        Err(e) => {
            println!(
                "{} Could not load saved server: {}",
                emoji::symbols::warning::WARNING.glyph,
                e
            );
            println!(
                "{} Assuming no server application exists...",
                INFORMATION.glyph
            );
            None
        }
    };

    update_server_app(existing_server_app, server, http_client).await
}

async fn update_server_app<S: Server<S, A>, A: ServerApplication<S, A>>(
    existing_server_app: Option<A>,
    server: &S,
    http_client: &Client,
) -> Option<A> {
    let default_server_app = server.default_version_check_client();
    let checking_server_app = match &existing_server_app {
        Some(server_app) => server_app,
        None => &default_server_app,
    };

    match checking_server_app
        .check_for_updated_server(server, http_client)
        .await
    {
        Ok(update_result) => {
            replace_server_app_if_new_one_exists(update_result, existing_server_app, http_client)
                .await
        }
        Err(e) => {
            println!(
                "{} Error occurred while checking for updated server: {}",
                CROSS_MARK.glyph, e
            );
            println!(
                "{} Attempting to roll back to existing server!",
                INFORMATION.glyph
            );
            existing_server_app
        }
    }
}

async fn replace_server_app_if_new_one_exists<S: Server<S, A>, A: ServerApplication<S, A>>(
    update_result: Option<A>,
    existing_server_app: Option<A>,
    http_client: &Client,
) -> Option<A> {
    match update_result {
        Some(updated_server_app) => match updated_server_app.download_server(http_client).await {
            Ok(_) => {
                if let Some(app) = existing_server_app {
                    match app.delete_server() {
                        Ok(_) => println!(
                            "{} Successfully deleted deprecated server app!",
                            CHECK_MARK.glyph
                        ),
                        Err(e) => println!(
                            "{} Failed to delete old server app: {}",
                            CROSS_MARK.glyph, e
                        ),
                    }
                }
                Some(updated_server_app)
            }
            Err(e) => {
                println!(
                    "{} Failed to download updated server: {}",
                    emoji::symbols::warning::WARNING.glyph,
                    e
                );
                existing_server_app
            }
        },
        None => existing_server_app,
    }
}

fn display_server_result(run_result: &crate::Result<ExitStatus>) {
    match run_result {
        Ok(result) => {
            println!("{} Server exited with: ({})", INFORMATION.glyph, result)
        }
        Err(e) => {
            println!("{} Server encountered an error: {}", CROSS_MARK.glyph, e)
        }
    }
}

fn save_server_info_if_exists<S: Server<S, A>, A: ServerApplication<S, A>>(
    server: &S,
    server_app: &Option<A>,
) {
    if let Some(client) = server_app {
        match client.save_server_info(server) {
            Ok(_) => println!("{} Successfully saved server info!", CHECK_MARK.glyph),
            Err(e) => println!("{} Unable to save server info: {}", CROSS_MARK.glyph, e),
        }
    }
}
