use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::shared::config::{
    minecraft::{papermc::PaperMcConfig, MinecraftConfig, MinecraftServerConfig},
    AppConfig, GameServerConfig,
};

use super::schema::prelude::*;

pub async fn server_config(
    id: Uuid,
    connection: &DatabaseConnection,
) -> anyhow::Result<Option<GameServerConfig>> {
    let server_row = GameServer::find_by_id(id.to_string())
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
