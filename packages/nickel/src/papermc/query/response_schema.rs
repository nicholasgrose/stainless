use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::Deserialize;

static PAPERMC_DOWNLOAD_TYPE_NAME: &str = "application";

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct VersionResponse {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<i32>,
}

impl VersionResponse {
    pub fn most_recent_build(&self) -> Option<&i32> {
        self.builds.iter().max()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct BuildResponse {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub build: i32,
    pub time: String,
    pub channel: String,
    pub promoted: bool,
    pub changes: Vec<Change>,
    pub downloads: HashMap<String, Download>,
}

impl BuildResponse {
    pub fn application_download(&self) -> Option<&Download> {
        self.downloads.get(PAPERMC_DOWNLOAD_TYPE_NAME)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Change {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Download {
    pub name: String,
    pub sha256: String,
}

impl Display for Download {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Name: {}, SHA256: {}}}", self.name, self.sha256)
    }
}
