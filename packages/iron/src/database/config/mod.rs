use crate::shared::config::{
    minecraft::{papermc::PaperMC, Minecraft, MinecraftServer},
    App, ServerConfig,
};

use super::{schema, Database};

impl Database {
    pub fn get_server_config(&self, _name: &str) -> crate::Result<Option<ServerConfig>> {
        let connection = self.connection_pool.get()?;

        Ok(None)
    }
}
