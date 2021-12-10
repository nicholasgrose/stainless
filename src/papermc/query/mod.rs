use anyhow::Error;
use reqwest::Client;

use crate::papermc::{PaperMCClient, PaperMCProject};
use crate::papermc::query::response_schema::{BuildResponse, Download, VersionResponse};

mod url;
mod response_schema;

pub async fn latest_papermc_client_for_project(project: PaperMCProject, client: &Client) -> Result<PaperMCClient, Error> {
    let latest_build = latest_project_build(&project, client).await?;
    let application_download = application_build_download(&project, &latest_build, client).await?;

    Ok(PaperMCClient {
        project,
        build: latest_build,
        download: application_download.name.clone(),
    })
}

async fn application_build_download(project: &PaperMCProject, latest_build: &i32, client: &Client) -> Result<Download, Error> {
    let build_response = call_papermc_project_build_api(&project, &latest_build, &client)
        .await?;

    match build_response.application_download() {
        Some(download) => Ok(download.clone()),
        None => Err(Error::msg("no client application downloads found for latest build of project")),
    }
}

async fn latest_project_build(project: &PaperMCProject, client: &Client) -> Result<i32, Error> {
    let build_response = call_papermc_project_version_api(&project, &client)
        .await?;

    match build_response.most_recent_build() {
        Some(build) => Ok(build.clone()),
        None => Err(Error::msg("no builds found for provided project")),
    }
}

async fn call_papermc_project_version_api(project: &PaperMCProject, client: &Client) -> Result<VersionResponse, Error> {
    Ok(client
        .get(url::papermc_project_version_url(project))
        .send()
        .await?
        .error_for_status()?
        .json::<VersionResponse>()
        .await?
    )
}

async fn call_papermc_project_build_api(project: &PaperMCProject, build: &i32, client: &Client) -> Result<BuildResponse, Error> {
    Ok(client
        .get(url::papermc_project_build_download_url(project, build))
        .send()
        .await?
        .error_for_status()?
        .json::<BuildResponse>()
        .await?
    )
}
