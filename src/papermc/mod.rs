use std::fmt::{Display, Formatter};
use std::fs::{File, remove_file};
use std::path::Path;
use std::process::{Command, Output};

use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use crate::{Client, CLIENT_INFO_DIR_PATH, ClientConfig};

pub mod query;

pub struct PaperMCConfig {
    pub project: PaperMCProject,
    pub java_arguments: Vec<String>,
}

impl ClientConfig for PaperMCConfig {}

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
pub struct PaperMCClient {
    pub project: PaperMCProject,
    pub build: i32,
    pub application_download: Download,
}

impl Display for PaperMCClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Project: {}, Build: {}, Download: {}}}", self.project, self.build, self.application_download)
    }
}

#[async_trait]
impl Client<PaperMCConfig, PaperMCClient> for PaperMCClient {
    async fn check_for_updated_client(&self, _config: &PaperMCConfig, http_client: &HttpClient) -> crate::Result<Option<PaperMCClient>> {
        let latest_client = query::latest_papermc_client_for_project(&self.project, http_client)
            .await?;

        if latest_client.build > self.build {
            println!("{} Newer client is available: {} over {}", emoji::symbols::other_symbol::CHECK_MARK.glyph, latest_client.build, self.build);
            Ok(Some(latest_client))
        } else {
            println!("{} No newer client is available!", emoji::symbols::other_symbol::CHECK_MARK.glyph);
            Ok(None)
        }
    }

    async fn download_client(&self, http_client: &HttpClient) -> crate::Result<()> {
        println!("{} Downloading {}...", emoji::symbols::alphanum::INFORMATION.glyph, self.application_download.name);

        query::download_application_client(
            self,
            Path::new(&self.application_download.name),
            http_client,
        ).await
    }

    fn delete_client(&self) -> crate::Result<()> {
        println!("{} Removing {}...", emoji::symbols::alphanum::INFORMATION.glyph, self.application_download.name);

        remove_file(Path::new(&self.application_download.name))?;

        Ok(())
    }

    fn save_client_info(&self) -> crate::Result<()> {
        let client_info_file = File::create(Path::new(&client_info_file_path()))?;
        bincode::serialize_into(client_info_file, self)?;

        Ok(())
    }

    fn delete_client_info(&self) -> crate::Result<()> {
        std::fs::remove_file(Path::new(&client_info_file_path()))?;

        Ok(())
    }

    fn start_client(&self, server_config: &PaperMCConfig) -> crate::Result<Output> {
        println!("{} Starting {}...", emoji::symbols::alphanum::INFORMATION.glyph, self.application_download.name);

        let server_output = Command::new("java")
            .arg("-jar")
            .arg(&self.application_download.name)
            .arg("nogui")
            .args(&server_config.java_arguments)
            .spawn()?
            .wait_with_output()?;

        Ok(server_output)
    }
}

impl PaperMCClient {
    pub fn default(project: &PaperMCProject) -> PaperMCClient {
        return PaperMCClient {
            project: project.clone(),
            build: -1,
            application_download: Download {
                name: String::from(""),
                sha256: vec!(),
            },
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct Download {
    pub name: String,
    pub sha256: Vec<u8>,
}

impl Display for Download {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Name: {}, SHA256: {}}}", self.name, hex::encode(&self.sha256))
    }
}

fn client_info_file_path() -> String {
    format!("{}/{}", CLIENT_INFO_DIR_PATH, "papermc")
}

pub fn load_saved_client() -> crate::Result<PaperMCClient> {
    let client_info_path = client_info_file_path();
    let saved_client_path = Path::new(&client_info_path);
    let saved_client_file = File::open(saved_client_path)?;

    Ok(bincode::deserialize_from(saved_client_file)?)
}
