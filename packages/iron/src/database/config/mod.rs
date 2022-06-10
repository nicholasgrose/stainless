use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection,
};

use crate::{
    database::schema::{ServerConfigRow, ServerTypeRow},
    shared::config::{
        minecraft::{papermc::PaperMC, Minecraft, MinecraftServer},
        App, ServerConfig,
    },
};

use super::{schema::sql, Database};

impl Database {
    pub fn server_config(&self, server_name: &str) -> crate::Result<Option<ServerConfig>> {
        let mut connection = self.connection_pool.get()?;
        let config_row = match server_config_row(server_name, &mut connection)? {
            Some(r) => r,
            None => return Ok(None),
        };
        let server_type_row = match server_type_row(server_name, &mut connection)? {
            Some(r) => r,
            None => return Ok(None),
        };

        Ok(None)
    }
}

fn server_config_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<ServerConfigRow>> {
    use sql::server_configs::dsl::*;

    Ok(server_configs
        .find(server_name)
        .first::<ServerConfigRow>(connection)
        .optional()?)
}

fn server_type_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<ServerTypeRow>> {
    use sql::server_types::dsl::*;

    Ok(server_types
        .filter(name.eq(server_name))
        .first::<ServerTypeRow>(connection)
        .optional()?)
}
