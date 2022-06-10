use diesel::{QueryDsl, RunQueryDsl};

use crate::{shared::config::{
    minecraft::{papermc::PaperMC, Minecraft, MinecraftServer},
    App, ServerConfig,
}, database::schema::ServerConfigRow};

use super::{Database, schema::sql};

impl Database {
    pub fn get_server_config(&self, server_name: &str) -> crate::Result<Option<ServerConfig>> {
        use sql::server_configs::dsl::*;

        let mut connection = self.connection_pool.get()?;
        let config_row = server_configs
            .find(server_name)
            .first::<ServerConfigRow>(&mut connection);

        Ok(None)
    }
}
