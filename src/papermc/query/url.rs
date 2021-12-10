use crate::papermc::PaperMCProject;

static PAPERMC_API_BASE_URL: &str = "https://papermc.io/api/v2";

pub fn papermc_project_version_url(project: &PaperMCProject) -> String {
    format!("{}/projects/{}/versions/{}", PAPERMC_API_BASE_URL, project.name, project.version)
}

pub fn papermc_project_build_download_url(project: &PaperMCProject, build: &i32) -> String {
    format!("{}/builds/{}", papermc_project_version_url(project), build)
}
