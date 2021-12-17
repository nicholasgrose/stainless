use crate::{PaperMCProject, PaperMCServer};

pub struct StainlessConfig {
    pub server: ServerType,
}

pub enum ServerType {
    PaperMC(PaperMCServer),
}

// TODO: Make this load from a config file
pub fn load_stainless_config() -> crate::Result<StainlessConfig> {
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
