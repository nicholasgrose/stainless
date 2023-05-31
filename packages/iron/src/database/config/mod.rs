use anyhow::anyhow;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};

use crate::database::DatabaseContext;
use crate::shared::config::{
    App,
    minecraft::{Minecraft, MinecraftServer, papermc::PaperMC}, ServerConfig,
};

use super::{
    schema::{self, sql}
};

impl DatabaseContext {
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
        let app = match app_for_type(server_name, &server_type_row.server_type, &mut connection)? {
            Some(a) => a,
            None => return Ok(None),
        };

        Ok(Some(ServerConfig {
            name: config_row.name,
            app,
        }))
    }
}

fn server_config_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<schema::ServerConfig>> {
    use sql::server_configs::dsl::*;

    Ok(server_configs
        .find(server_name)
        .first(connection)
        .optional()?)
}

fn server_type_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<schema::ServerType>> {
    use sql::server_types::dsl::*;

    Ok(server_types
        .filter(name.eq(server_name))
        .first(connection)
        .optional()?)
}

fn app_for_type(
    server_name: &str,
    server_type: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<App>> {
    match server_type {
        "Minecraft" => minecraft_app(server_name, connection),
        _ => Err(anyhow!("invalid server type found").into()),
    }
}

fn minecraft_app(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<App>> {
    let minecraft_row = match minecraft_server_row(server_name, connection)? {
        Some(r) => r,
        None => return Ok(None),
    };
    let argument_rows = match minecraft_jvm_argument_rows(server_name, connection)? {
        Some(r) => r,
        None => return Ok(None),
    };
    let minecraft_type_row = match minecraft_server_type_row(server_name, connection)? {
        Some(r) => r,
        None => return Ok(None),
    };
    let minecraft_server = match minecraft_server_for_type(
        server_name,
        &minecraft_type_row.minecraft_server_type,
        connection,
    )? {
        Some(r) => r,
        None => return Ok(None),
    };

    Ok(Some(App::Minecraft(Minecraft {
        game_version: minecraft_row.game_version,
        jvm_runtime_arguments: argument_rows.into_iter().map(|r| r.argument).collect(),
        server: minecraft_server,
    })))
}

fn minecraft_jvm_argument_rows(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<Vec<schema::MinecraftJvmArgument>>> {
    use sql::minecraft_jvm_arguments::dsl::*;

    Ok(minecraft_jvm_arguments
        .filter(name.eq(server_name))
        .load(connection)
        .optional()?)
}

fn minecraft_server_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<schema::MinecraftServer>> {
    use sql::minecraft_servers::dsl::*;

    Ok(minecraft_servers
        .select(schema::MinecraftServer::as_select())
        .find(server_name)
        .first(connection)
        .optional()?)
}

fn minecraft_server_type_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<schema::MinecraftType>> {
    use sql::minecraft_types::dsl::*;

    Ok(minecraft_types
        .filter(name.eq(server_name))
        .first(connection)
        .optional()?)
}

fn minecraft_server_for_type(
    server_name: &str,
    minecraft_server_type: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<MinecraftServer>> {
    match minecraft_server_type {
        "PaperMC" => papermc_minecraft_server(server_name, connection),
        _ => Err(anyhow!("invalid minecraft server type found").into()),
    }
}

fn papermc_minecraft_server(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<MinecraftServer>> {
    let papermc_row = match papermc_row(server_name, connection)? {
        Some(r) => r,
        None => return Ok(None),
    };

    Ok(Some(MinecraftServer::PaperMC(PaperMC {
        project: papermc_row.project,
        build: papermc_row.build,
    })))
}

fn papermc_row(
    server_name: &str,
    connection: &mut SqliteConnection,
) -> crate::Result<Option<schema::PapermcServer>> {
    use sql::papermc_servers::dsl::*;

    Ok(papermc_servers
        .select(schema::PapermcServer::as_select())
        .find(server_name)
        .first(connection)
        .optional()?)
}
