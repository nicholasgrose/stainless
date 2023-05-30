use std::fmt::{Display, Formatter};
use std::fs::{remove_file, File};
use std::path::Path;
use std::process::{ExitStatus, Stdio};

use anyhow::Error;
use async_trait::async_trait;
use emoji::symbols::alphanum::INFORMATION;
use emoji::symbols::other_symbol::{CHECK_MARK, CROSS_MARK};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::mpsc::Receiver;
use tokio::{pin, select};

use crate::config::constants::SERVER_INFO_DIR_PATH;
use crate::server::{Server, ServerApplication};

pub mod query;

#[derive(Serialize, Deserialize)]
pub struct PaperMCServer {
    pub server_name: String,
    pub project: PaperMCProject,
    pub jvm_arguments: Vec<String>,
}

impl Server<PaperMCServer, PaperMCServerApp> for PaperMCServer {
    fn server_name(&self) -> &str {
        self.server_name.as_str()
    }

    fn jvm_arguments(&self) -> &Vec<String> {
        self.jvm_arguments.as_ref()
    }

    fn load_saved_server_app(&self) -> crate::Result<PaperMCServerApp> {
        let client_info_path = self.client_info_file_path();
        let saved_client_path = Path::new(&client_info_path);
        let mut saved_client_file = File::open(saved_client_path)?;
        let save_config = bincode::config::standard();
        let saved_client: PaperMCServerApp =
            bincode::serde::decode_from_std_read(&mut saved_client_file, save_config)?;

        println!(
            "{} Found existing server: {}",
            CHECK_MARK.glyph,
            saved_client.application_name()
        );

        Ok(saved_client)
    }

    fn client_info_file_path(&self) -> String {
        format!("{}/{}", SERVER_INFO_DIR_PATH, self.server_name)
    }

    fn default_version_check_client(&self) -> PaperMCServerApp {
        PaperMCServerApp::default(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaperMCProject {
    pub name: String,
    pub version: String,
}

impl Display for PaperMCProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Name: {}, Version: {}}}", self.name, self.version)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaperMCServerApp {
    pub project: PaperMCProject,
    pub build: i32,
    pub application_download: Download,
}

impl Display for PaperMCServerApp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{Project: {}, Build: {}, Download: {}}}",
            self.project, self.build, self.application_download
        )
    }
}

#[async_trait]
impl ServerApplication<PaperMCServer, PaperMCServerApp> for PaperMCServerApp {
    fn application_name(&self) -> &str {
        &self.application_download.name
    }

    async fn check_for_updated_server(
        &self,
        _config: &PaperMCServer,
        http_client: &Client,
    ) -> crate::Result<Option<PaperMCServerApp>> {
        let latest_client =
            query::latest_papermc_server_for_project(&self.project, http_client).await?;

        if latest_client.build > self.build {
            println!(
                "{} Newer server build is available: {}",
                CHECK_MARK.glyph, latest_client.build
            );
            Ok(Some(latest_client))
        } else {
            println!("{} No newer server is available!", CHECK_MARK.glyph);
            Ok(None)
        }
    }

    async fn download_server(&self, http_client: &Client) -> crate::Result<()> {
        println!(
            "{} Downloading {}...",
            INFORMATION.glyph,
            self.application_name()
        );

        query::download_server_application(self, Path::new(&self.application_name()), http_client)
            .await
    }

    fn delete_server(&self) -> crate::Result<()> {
        println!(
            "{} Removing {}...",
            INFORMATION.glyph,
            self.application_name()
        );

        remove_file(Path::new(&self.application_name()))?;

        Ok(())
    }

    fn save_server_info(&self, client_config: &PaperMCServer) -> crate::Result<()> {
        let mut client_info_file = File::create(Path::new(&client_config.client_info_file_path()))?;
        let save_config = bincode::config::standard();
        bincode::serde::encode_into_std_write(self, &mut client_info_file, save_config)?;

        Ok(())
    }

    fn delete_server_info(&self, client_config: &PaperMCServer) -> crate::Result<()> {
        std::fs::remove_file(Path::new(&client_config.client_info_file_path()))?;

        Ok(())
    }

    async fn start_server(
        &self,
        server_config: &PaperMCServer,
        input_receiver: &mut Receiver<u8>,
    ) -> crate::Result<ExitStatus> {
        println!(
            "{} Starting {}...",
            INFORMATION.glyph,
            self.application_name()
        );

        let mut server_process = Command::new("java")
            .arg("-jar")
            .arg(&self.application_name())
            .arg("nogui")
            .args(server_config.jvm_arguments())
            .stdin(Stdio::piped())
            .spawn()?;

        let mut child_in = server_process.stdin.take().unwrap();

        let server_task = server_process.wait();
        pin!(server_task);

        loop {
            select! {
                receive_result = input_receiver.recv() => {
                    match receive_result {
                        Some(byte) => child_in.write_u8(byte).await?,
                        None => return Err(Error::msg("Input channel broke."))
                    }
                }
                server_result = &mut server_task => {
                    match server_result {
                        Ok(output) => {
                            return Ok(output)
                        },
                        Err(e) => {
                            println!("{} Error occurred running server: {}", CROSS_MARK.glyph, e);
                            return Err(Error::from(e))
                        }
                    }
                }
            }
        }
    }
}

impl PaperMCServerApp {
    pub fn default(config: &PaperMCServer) -> PaperMCServerApp {
        PaperMCServerApp {
            project: config.project.clone(),
            build: -1,
            application_download: Download {
                name: String::from(""),
                sha256: vec![],
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Download {
    pub name: String,
    pub sha256: Vec<u8>,
}

impl Display for Download {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{Name: {}, SHA256: {}}}",
            self.name,
            hex::encode(&self.sha256)
        )
    }
}
