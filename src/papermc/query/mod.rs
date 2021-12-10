use anyhow::Error;
use reqwest::Client;

use crate::papermc::{PaperMCClient, PaperMCProject};
use crate::papermc::query::schema::{ProjectBuildResponse, ProjectVersionResponse};
use crate::papermc::query::url::{papermc_project_build_download_url, papermc_project_version_url};

mod url;
mod schema;

static PAPERMC_DOWNLOAD_TYPE_NAME: &str = "application";

pub async fn latest_papermc_client_for_project(project: PaperMCProject, client: &Client) -> Result<PaperMCClient, Error> {
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
        Err(Error::msg("no client application downloads provided for latest build of project"))
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
