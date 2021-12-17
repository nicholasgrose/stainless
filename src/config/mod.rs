use crate::{PaperMCProject, PaperMCServer};
use crate::config::constants::SERVER_INFO_DIR_PATH;

pub mod constants;

pub struct StainlessConfig {
    pub server: ServerType,
}

pub enum ServerType {
    PaperMC(PaperMCServer),
}

// TODO: Make this load from a config file
pub fn load_stainless_config() -> crate::Result<StainlessConfig> {
    std::fs::create_dir_all(SERVER_INFO_DIR_PATH)?;

    println!("{} Loading server configuration...", emoji::symbols::alphanum::INFORMATION.glyph);

    println!("{} Stainless configuration loaded!", emoji::symbols::other_symbol::CHECK_MARK.glyph);

    Ok(StainlessConfig {
        server: ServerType::PaperMC(PaperMCServer {
            server_name: "papermc".to_string(),
            project: PaperMCProject {
                name: "paper".to_string(),
                version: "1.18.1".to_string(),
            },
            jvm_arguments: vec!(),
        })
    })
}
