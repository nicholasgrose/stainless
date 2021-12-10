use tokio;

use crate::papermc::PaperMCProject;

mod papermc;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();
    let project = PaperMCProject {
        name: String::from("paper"),
        version: String::from("1.18"),
    };

    let latest_client = papermc::query::latest_papermc_client_for_project(project, &http_client)
        .await;

    match latest_client {
        Ok(client) => println!("{}", client),
        Err(error) => println!("{}", error),
    };
}
