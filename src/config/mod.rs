use config::{Config, File};
use emoji::symbols::alphanum::INFORMATION;
use emoji::symbols::other_symbol::CHECK_MARK;
use serde::{Deserialize, Serialize};

use crate::PaperMCServer;
use crate::config::constants::{SERVER_INFO_DIR_PATH, STAINLESS_CONFIG_PATH};

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
pub fn load_stainless_config() -> crate::Result<StainlessConfig> {
    std::fs::create_dir_all(SERVER_INFO_DIR_PATH)?;

    println!("{} Loading server configuration...", INFORMATION.glyph);

    let mut config = Config::default();
    config.merge(File::with_name(STAINLESS_CONFIG_PATH))?;
    let result: StainlessConfig = config.try_into()?;

    println!("{} Stainless configuration loaded!", CHECK_MARK.glyph);

    Ok(result)
}
