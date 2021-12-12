use std::fmt::{Display, Formatter};
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use anyhow::Error;
use reqwest::Client;

use crate::ServerConfig;

pub mod query;

pub struct PaperMCProject {
    pub name: String,
    pub version: String,
}

impl Display for PaperMCProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Name: {}, Version: {}}}", self.name, self.version)
    }
}

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

impl PaperMCClient {
    pub async fn download_client(&self, client_dir_path: &str, http_client: &Client) -> Result<(), Error> {
        let client_dir_path = Path::new(client_dir_path);
        let client_file_path = self.client_file_path(client_dir_path);

        query::download_application_client(
            self,
            client_file_path.as_path(),
            http_client,
        ).await
    }

    fn client_file_path(&self, client_dir_path: &Path) -> PathBuf {
        client_dir_path.join(Path::new(&self.application_download.name))
    }

    pub fn delete_client(&self, client_dir_path: &str) -> Result<(), Error> {
        let client_dir_path = Path::new(client_dir_path);
        remove_file(self.client_file_path(client_dir_path))?;

        Ok(())
    }

    pub fn start_server(&self, server_config: &ServerConfig) -> Result<ExitStatus, Error> {
        let client_dir_path = Path::new(&server_config.client_dir_path);
        let server_output = Command::new("java")
            .arg("-jar")
            .arg(self.client_file_path(client_dir_path))
            .args(&server_config.java_arguments)
            .spawn()?
            .wait_with_output()?;
        Ok(server_output.status)
    }
}

pub struct Download {
    pub name: String,
    pub sha256: Vec<u8>,
}

impl Display for Download {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Name: {}, SHA256: {}}}", self.name, hex::encode(&self.sha256))
    }
}
