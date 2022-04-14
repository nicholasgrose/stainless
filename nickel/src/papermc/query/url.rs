use crate::papermc::PaperMCProject;
use crate::server::ServerApplication;
use crate::PaperMCServerApp;

static PAPERMC_API_BASE_URL: &str = "https://papermc.io/api/v2";

pub fn papermc_project_version_url(project: &PaperMCProject) -> String {
    format!(
        "{}/projects/{}/versions/{}",
        PAPERMC_API_BASE_URL, project.name, project.version
    )
}

pub fn papermc_project_build_url(project: &PaperMCProject, build: i32) -> String {
    format!("{}/builds/{}", papermc_project_version_url(project), build)
}

pub fn papermc_project_download_url(server_app: &PaperMCServerApp) -> String {
    format!(
        "{}/downloads/{}",
        papermc_project_build_url(&server_app.project, server_app.build),
        server_app.application_name()
    )
}
