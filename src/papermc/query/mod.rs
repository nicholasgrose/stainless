use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use reqwest::Client;
use sha2::{Digest, Sha256};

use crate::papermc::{Download, PaperMCClient, PaperMCProject};
use crate::papermc::query::response_schema::{BuildResponse, Download as SchemaDownload, VersionResponse};

mod url;
mod response_schema;

pub async fn latest_papermc_client_for_project(project: PaperMCProject, http_client: &Client) -> Result<PaperMCClient, Error> {
    let latest_build = latest_project_build(&project, http_client).await?;
    let application_download = application_build_download(&project, &latest_build, http_client).await?;

    Ok(PaperMCClient {
        project,
        build: latest_build,
        application_download: Download {
            name: application_download.name,
            sha256: hex::decode(application_download.sha256)?,
        },
    })
}

async fn application_build_download(project: &PaperMCProject, latest_build: &i32, http_client: &Client) -> Result<SchemaDownload, Error> {
    let build_response = call_papermc_project_build_api(&project, &latest_build, &http_client)
        .await?;

    match build_response.application_download() {
        Some(download) => Ok(download.clone()),
        None => Err(Error::msg("no client application downloads found for latest build of project")),
    }
}

async fn latest_project_build(project: &PaperMCProject, http_client: &Client) -> Result<i32, Error> {
    let build_response = call_papermc_project_version_api(&project, &http_client)
        .await?;

    match build_response.most_recent_build() {
        Some(build) => Ok(build.clone()),
        None => Err(Error::msg("no builds found for provided project")),
    }
}

async fn call_papermc_project_version_api(project: &PaperMCProject, http_client: &Client) -> Result<VersionResponse, Error> {
    Ok(http_client
        .get(url::papermc_project_version_url(project))
        .send()
        .await?
        .error_for_status()?
        .json::<VersionResponse>()
        .await?
    )
}

async fn call_papermc_project_build_api(project: &PaperMCProject, build: &i32, http_client: &Client) -> Result<BuildResponse, Error> {
    Ok(http_client
        .get(url::papermc_project_build_url(project, build))
        .send()
        .await?
        .error_for_status()?
        .json::<BuildResponse>()
        .await?
    )
}

pub async fn download_application_client(project: &PaperMCClient, client_file_path: &Path, http_client: &Client) -> Result<(), Error> {
    let mut response = http_client
        .get(url::papermc_project_download_url(project))
        .send()
        .await?
        .error_for_status()?;

    let mut client_file = File::create(client_file_path)?;
    let mut hasher = Sha256::default();

    loop {
        let chunk = match response.chunk().await? {
            Some(chunk) => chunk,
            None => break,
        };

        hasher.write_all(&chunk)?;
        client_file.write_all(&chunk)?;
    }

    let hash = hasher.finalize();
    if hash[..] == project.application_download.sha256 {
        Ok(())
    } else {
        Err(Error::msg("download does not match hash"))
    }

    // let hash = digest_bytes(&response_bytes);
    //
    // if hash == project.application_download.sha256 {
    //     let path = format!("{}/{}", path, project.application_download.name);
    //     let path = Path::new(&path);
    //     File::create(path)?
    //         .write_all(&response_bytes);
    //
    //     Ok(())
    // } else {
    //     Err(Error::msg("download does not match hash"))
    // }
}