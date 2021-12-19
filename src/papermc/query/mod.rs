use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use emoji::symbols::other_symbol::{CHECK_MARK, CROSS_MARK};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use sha2::{Digest, Sha256};

use crate::papermc::{Download, PaperMCProject, PaperMCServerApp};
use crate::papermc::query::response_schema::{BuildResponse, Download as SchemaDownload, VersionResponse};

mod url;
mod response_schema;

pub async fn latest_papermc_server_for_project(project: &PaperMCProject, http_client: &Client) -> crate::Result<PaperMCServerApp> {
    let latest_build = latest_project_build(project, http_client).await?;
    let application_download = application_build_download(project, &latest_build, http_client).await?;

    Ok(PaperMCServerApp {
        project: project.clone(),
        build: latest_build,
        application_download: Download {
            name: application_download.name,
            sha256: hex::decode(application_download.sha256)?,
        },
    })
}

async fn application_build_download(project: &PaperMCProject, latest_build: &i32, http_client: &Client) -> crate::Result<SchemaDownload> {
    let build_response = call_papermc_project_build_api(&project, &latest_build, &http_client)
        .await?;

    match build_response.application_download() {
        Some(download) => Ok(download.clone()),
        None => Err(Error::msg("no server application downloads found for latest build of project")),
    }
}

async fn latest_project_build(project: &PaperMCProject, http_client: &Client) -> crate::Result<i32> {
    let build_response = call_papermc_project_version_api(&project, &http_client)
        .await?;

    match build_response.most_recent_build() {
        Some(build) => Ok(build.clone()),
        None => Err(Error::msg("no builds found for provided papermc project")),
    }
}

async fn call_papermc_project_version_api(project: &PaperMCProject, http_client: &Client) -> crate::Result<VersionResponse> {
    Ok(http_client
        .get(url::papermc_project_version_url(project))
        .send()
        .await?
        .error_for_status()?
        .json::<VersionResponse>()
        .await?
    )
}

async fn call_papermc_project_build_api(project: &PaperMCProject, build: &i32, http_client: &Client) -> crate::Result<BuildResponse> {
    Ok(http_client
        .get(url::papermc_project_build_url(project, build))
        .send()
        .await?
        .error_for_status()?
        .json::<BuildResponse>()
        .await?
    )
}

pub async fn download_server_application(project: &PaperMCServerApp, client_file_path: &Path, http_client: &Client) -> crate::Result<()> {
    let mut response = http_client
        .get(url::papermc_project_download_url(project))
        .send()
        .await?
        .error_for_status()?;

    let content_length = match response.content_length() {
        Some(len) => len,
        None => return Err(Error::msg("no content delivered for download")),
    };
    let progress_bar = ProgressBar::new(content_length);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes:.1f}/{total_bytes:.1f} ({bytes_per_sec}) {msg}")
    );
    let mut client_file = File::create(client_file_path)?;
    let mut hasher = Sha256::default();

    progress_bar.set_message("Downloading...");
    loop {
        let chunk = match response.chunk().await? {
            Some(chunk) => chunk,
            None => break,
        };

        progress_bar.inc(chunk.len() as u64);
        hasher.write_all(&chunk)?;
        client_file.write_all(&chunk)?;
    }

    progress_bar.finish_with_message("Done");
    let hash = hasher.finalize();
    if hash[..] == project.application_download.sha256 {
        println!("{} Download checksum correct!", CHECK_MARK.glyph);
        Ok(())
    } else {
        println!("{} Download checksum does not match!", CROSS_MARK.glyph);
        Err(Error::msg("download does not match hash"))
    }
}