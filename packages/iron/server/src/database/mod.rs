use anyhow::Context;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use entity::application::ActiveModel as Application;
use entity::minecraft_server::ActiveModel as MinecraftServer;
use entity::paper_mc_server::ActiveModel as PaperMcServer;
use iron_api::minecraft_service::{PaperMcProject, PaperMcServerDefinition};

pub async fn save_paper_mc_server(
    db: &DatabaseConnection,
    papermc_definition: &PaperMcServerDefinition,
) -> anyhow::Result<String> {
    let minecraft_server_definition = papermc_definition
        .minecraft_server_definition
        .as_ref()
        .with_context(|| "no minecraft server definition provided")?;
    let server_definition = minecraft_server_definition
        .server_definition
        .as_ref()
        .with_context(|| "no server definition provided")?;

    let paper_mc_project = PaperMcProject::from_i32(papermc_definition.project)
        .with_context(|| "invalid paper mc project provided")?;

    let new_server_id = Uuid::new_v4();
    let id_string = new_server_id.to_string();

    let transaction = db.begin().await?;

    Application {
        id: Set(id_string.clone()),
        name: Set(server_definition.name.clone()),
        command: Set("".to_string()),
        active: Set(server_definition.active),
    }
    .insert(&transaction)
    .await?;

    MinecraftServer {
        id: Set(id_string.clone()),
        game_version: Set(minecraft_server_definition.game_version.clone()),
    }
    .insert(&transaction)
    .await?;

    PaperMcServer {
        id: Set(id_string.clone()),
        project: Set(paper_mc_project.as_str_name().to_string()),
        build: Set(0),
        build_update_off: Set(false),
    }
    .insert(&transaction)
    .await?;

    transaction.commit().await?;

    Ok(new_server_id.to_string())
}
