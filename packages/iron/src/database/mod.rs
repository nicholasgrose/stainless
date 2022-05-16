use std::collections::HashMap;

use velcro::hash_map;

use crate::config::{
    minecraft::{papermc::PaperMC, Minecraft, MinecraftServer},
    Game, ServerConfig,
};

#[derive(Default, Clone)]
pub struct Database {
    servers: HashMap<String, ServerConfig>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            servers: hash_map! {
                "test".to_string(): ServerConfig {
                    name: "test".to_string(),
                    app: Game::Minecraft( Minecraft {
                        jvm_runtime_arguments: vec!["-Xmx:4G".to_string()],
                        game_version: "1.18.2".to_string(),
                        server: MinecraftServer::PaperMC(PaperMC {
                            project: "Paper".to_string(),
                            build: 69
                        })
                    })
                },
                "test1".to_string(): ServerConfig {
                    name: "test1".to_string(),
                    app: Game::Minecraft(Minecraft {
                        jvm_runtime_arguments: vec!["-Xmx:2G".to_string()],
                        game_version: "1.16.5".to_string(),
                        server: MinecraftServer::PaperMC(PaperMC {
                            project: "Paper".to_string(),
                            build: 68
                        })
                    })
                }
            },
        }
    }

    pub fn get_server(&self, name: &str) -> Option<&ServerConfig> {
        self.servers.get(name)
    }
}

impl juniper::Context for Database {}
