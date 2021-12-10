use std::collections::HashMap;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ProjectVersionResponse {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<i32>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ProjectBuildResponse {
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

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Change {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Download {
    pub name: String,
    pub sha256: String,
}
