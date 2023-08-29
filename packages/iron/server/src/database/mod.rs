use anyhow::Context;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, TransactionTrait};
use tracing::{info, instrument};

use entity::application::ActiveModel as Application;
use entity::minecraft_server::ActiveModel as MinecraftServer;
use entity::paper_mc_server::ActiveModel as PaperMcServer;
use iron_api::minecraft_service::{PaperMcProject, PaperMcServerDefinition};

use crate::web::grpc::AppCreateContext;

#[derive(Debug)]
pub struct IronDatabase {
    connection: DatabaseConnection,
}

impl From<DatabaseConnection> for IronDatabase {
    fn from(connection: DatabaseConnection) -> Self {
        IronDatabase { connection }
    }
}

impl IronDatabase {
    #[instrument]
    pub async fn save_paper_mc_server(
        &self,
        context: &AppCreateContext<PaperMcServerDefinition>,
    ) -> anyhow::Result<()> {
        info!("saving to database");

        let minecraft_server_definition =
            required_def!(context.message.minecraft_server_definition)?;
        let server_definition = required_def!(minecraft_server_definition.server_definition)?;

        let paper_mc_project = PaperMcProject::from_i32(context.message.project)
            .with_context(|| "invalid paper mc project provided")?;

        let transaction = self.connection.begin().await?;

        Application {
            id: Set(context.application.id.to_string()),
            name: Set(server_definition.name.clone()),
            command: Set(context.application.command.clone()),
            active: Set(server_definition.active),
        }
        .insert(&transaction)
        .await?;

        MinecraftServer {
            id: Set(context.application.id.to_string()),
            game_version: Set(minecraft_server_definition.game_version.clone()),
        }
        .insert(&transaction)
        .await?;

        PaperMcServer {
            id: Set(context.application.id.to_string()),
            project: Set(paper_mc_project.as_str_name().to_string()),
            build: Set(0),
            build_update_off: Set(false),
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
