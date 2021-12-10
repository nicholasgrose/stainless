use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use anyhow::Error;
use reqwest::Client;
use serde::Deserialize;
use tokio;

static PAPERMC_URL_ROOT: &str = "https://papermc.io/api/v2";
static PAPERMC_DOWNLOAD_TYPE_NAME: &str = "application";

struct PaperMCProject {
    name: String,
    version: String,
}

impl Display for PaperMCProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Name: {}, Version: {}}}", self.name, self.version)
    }
}

struct PaperMCClient {
    project: PaperMCProject,
    build: i32,
    download: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ProjectVersionResponse {
    project_id: String,
    project_name: String,
    version: String,
    builds: Vec<i32>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ProjectBuildResponse {
    project_id: String,
    project_name: String,
    version: String,
    build: i32,
    time: String,
    channel: String,
    promoted: bool,
    changes: Vec<Change>,
    downloads: HashMap<String, Download>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Change {
    commit: String,
    summary: String,
    message: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Download {
    name: String,
    sha256: String,
}

impl Display for PaperMCClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Project: {}, Build: {}, Download: {}}}", self.project, self.build, self.download)
    }
}

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();
    let project = PaperMCProject {
        name: String::from("paper"),
        version: String::from("1.18"),
    };
    let latest_client = latest_papermc_client_for_project(project, &http_client)
        .await;

    match latest_client {
        Ok(client) => println!("{}", client),
        Err(error) => println!("{}", error),
    }
}

async fn latest_papermc_client_for_project(project: PaperMCProject, client: &Client) -> Result<PaperMCClient, Error> {
    let build_response = call_papermc_project_version_api(&project, &client)
        .await?;
    let latest_build = build_response.builds[0];
    let build_response = call_papermc_project_build_api(&project, &latest_build, &client)
        .await?;

    if let Some(application_download) = build_response.downloads.get(PAPERMC_DOWNLOAD_TYPE_NAME) {
        Ok(PaperMCClient {
            project,
            build: latest_build,
            download: String::from(&application_download.name),
        })
    } else {
        Err(Error::msg("no available downloads provided for latest build of project"))
    }
}

async fn call_papermc_project_version_api(project: &PaperMCProject, client: &Client) -> Result<ProjectVersionResponse, Error> {
    Ok(client
        .get(papermc_project_version_url(project))
        .send()
        .await?
        .error_for_status()?
        .json::<ProjectVersionResponse>()
        .await?
    )
}

fn papermc_project_version_url(project: &PaperMCProject) -> String {
    format!("{}/projects/{}/versions/{}", PAPERMC_URL_ROOT, project.name, project.version)
}

async fn call_papermc_project_build_api(project: &PaperMCProject, build: &i32, client: &Client) -> Result<ProjectBuildResponse, Error> {
    Ok(client
        .get(papermc_project_build_download_url(project, build))
        .send()
        .await?
        .error_for_status()?
        .json::<ProjectBuildResponse>()
        .await?
    )
}

fn papermc_project_build_download_url(project: &PaperMCProject, build: &i32) -> String {
    format!("{}/builds/{}", papermc_project_version_url(project), build)
}
