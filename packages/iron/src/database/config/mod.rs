use crate::shared::config::{
    minecraft::{papermc::PaperMC, Minecraft, MinecraftServer},
    App, ServerConfig,
};

use super::Database;

impl Database {
    pub fn get_server_config(&self, name: &str) -> crate::Result<Option<ServerConfig>> {
        let connection = self.connection_pool.get()?;

        Ok(None)
    }
}
