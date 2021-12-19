use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;
use anyhow::Error;
use config::{Config};
use emoji::symbols::alphanum::INFORMATION;
use emoji::symbols::other_symbol::CHECK_MARK;
use emoji::symbols::warning::WARNING;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::PaperMCServer;
use crate::config::constants::{DOWNLOAD_PROGRESS_BAR_TEMPLATE, SERVER_INFO_DIR_PATH, STAINLESS_CONFIG_PATH, STAINLESS_DEFAULT_CONFIG_URL};

pub mod constants;

pub type StainlessConfig = Stainless;

#[derive(Serialize, Deserialize)]
pub struct Stainless {
    pub server: ServerType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    PaperMC(PaperMCServer),
}

// TODO: Make this load from a config file
pub async fn load_stainless_config(http_client: &Client) -> crate::Result<StainlessConfig> {
    generate_stainless_files_and_directories(http_client).await?;

    println!("{} Loading server configuration...", INFORMATION.glyph);

    let mut config = Config::default();
    config.merge(config::File::with_name(STAINLESS_CONFIG_PATH))?;
    let result: StainlessConfig = config.try_into()?;

    println!("{} Stainless configuration loaded!", CHECK_MARK.glyph);

    Ok(result)
}

async fn generate_stainless_files_and_directories(http_client: &Client) -> crate::Result<()> {
    println!("{} Generating any missing Stainless files or directories...", INFORMATION.glyph);

    generate_stainless_directories()?;
    generate_stainless_config_file_if_needed(http_client).await?;

    Ok(())
}

fn generate_stainless_directories() -> crate::Result<()> {
    std::fs::create_dir_all(SERVER_INFO_DIR_PATH)?;

    Ok(())
}

async fn generate_stainless_config_file_if_needed(http_client: &Client) -> crate::Result<()> {
    let config_path = Path::new(STAINLESS_CONFIG_PATH);

    if let Err(e) = fs::File::open(config_path) {
        if e.kind() == ErrorKind::NotFound {
            println!("{} Could not find existing config file.", WARNING.glyph);

            generate_new_stainless_config_file(http_client, config_path).await?
        }

        return Err(Error::from(e))
    }

    Ok(())
}

async fn generate_new_stainless_config_file(http_client: &Client, config_path: &Path) -> crate::Result<()> {
    println!("{} Attempting to create new config file...", INFORMATION.glyph);

    let mut config_file = fs::File::create(config_path)?;
    let mut default_config_response = http_client.get(STAINLESS_DEFAULT_CONFIG_URL)
        .send()
        .await?;

    let content_length = match default_config_response.content_length() {
        Some(len) => len,
        None => return Err(Error::msg("no content delivered for download")),
    };
    let progress_bar = ProgressBar::new(content_length);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template(DOWNLOAD_PROGRESS_BAR_TEMPLATE)
    );

    progress_bar.set_message("Downloading...");
    loop {
        let chunk = match default_config_response.chunk().await? {
            Some(chunk) => chunk,
            None => break,
        };

        progress_bar.inc(chunk.len() as u64);
        config_file.write_all(&chunk)?;
    }

    progress_bar.finish_with_message("Done");
    println!("{} Successfully created new configuration file!", CHECK_MARK.glyph);

    Ok(())
}
