use std::fmt::{Display, Formatter};
use std::fs::remove_file;
use std::path::Path;
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
    pub async fn download_client(&self, http_client: &Client) -> Result<(), Error> {
        println!("{} Downloading {}...", emoji::symbols::alphanum::INFORMATION.glyph, self.application_download.name);

        query::download_application_client(
            self,
            Path::new(&self.application_download.name),
            http_client,
        ).await
    }

    pub fn delete_client(&self) -> Result<(), Error> {
        println!("{} Removing {}...", emoji::symbols::alphanum::INFORMATION.glyph, self.application_download.name);

        remove_file(Path::new(&self.application_download.name))?;

        Ok(())
    }

    pub fn start_server(&self, server_config: &ServerConfig) -> Result<ExitStatus, Error> {
        println!("{} Starting {}...", emoji::symbols::alphanum::INFORMATION.glyph, self.application_download.name);

        let server_output = Command::new("java")
            .arg("-jar")
            .arg(&self.application_download.name)
            .arg("nogui")
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
