use sea_orm::{DatabaseConnection, EntityTrait};

use crate::shared::config::{
    AppConfig,
    GameServerConfig, minecraft::{MinecraftConfig, MinecraftServerConfig, papermc::PaperMcConfig},
};

use super::schema::prelude::*;

pub async fn server_config(
    id: i32,
    connection: &DatabaseConnection,
) -> anyhow::Result<Option<GameServerConfig>> {
    let server_row = GameServer::find_by_id(id)
        .left_join(MinecraftServer)
        .left_join(PaperMcServer)
        .one(connection)
        .await?;

    match server_row {
        None => Ok(None),
        Some(server) => Ok(Some(GameServerConfig {
            name: server.name,
            app: AppConfig::Minecraft(MinecraftConfig {
                jvm_runtime_arguments: vec![],
                game_version: "1".to_string(),
                server: MinecraftServerConfig::PaperMC(PaperMcConfig {
                    project: "paper".to_string(),
                    build: 2,
                }),
            }),
        })),
    }
}
