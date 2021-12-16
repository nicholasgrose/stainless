use crate::papermc::PaperMCProject;
use crate::PaperMCServer;

static PAPERMC_API_BASE_URL: &str = "https://papermc.io/api/v2";

pub fn papermc_project_version_url(project: &PaperMCProject) -> String {
    format!("{}/projects/{}/versions/{}", PAPERMC_API_BASE_URL, project.name, project.version)
}

pub fn papermc_project_build_url(project: &PaperMCProject, build: &i32) -> String {
    format!("{}/builds/{}", papermc_project_version_url(project), build)
}

pub fn papermc_project_download_url(server_client: &PaperMCServer) -> String {
    format!("{}/downloads/{}", papermc_project_build_url(&server_client.project, &server_client.build), server_client.application_download.name)
}
