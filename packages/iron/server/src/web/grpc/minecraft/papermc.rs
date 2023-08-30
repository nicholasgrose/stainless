use anyhow::Context;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

use entity::paper_mc_server::ActiveModel as PaperMcServerModel;
use iron_api::minecraft_service::{PaperMcProject, PaperMcServerDefinition};

use crate::database::insert::{Insert, InsertModel};
use crate::manager::Application;
use crate::web::grpc::minecraft::aikars_flags::AikarsFlags;
use crate::web::grpc::AppCreateContext;

impl TryFrom<PaperMcServerDefinition> for AppCreateContext<PaperMcServerDefinition> {
    type Error = anyhow::Error;

    fn try_from(papermc_definition: PaperMcServerDefinition) -> Result<Self, Self::Error> {
        let minecraft_server_definition =
            required_def!(papermc_definition.minecraft_server_definition)?;
        let server_definition = required_def!(minecraft_server_definition.server_definition)?;

        Ok(AppCreateContext {
            application: Application {
                id: Uuid::new_v4(),
                name: server_definition.name.clone(),
                command: AikarsFlags::try_from(minecraft_server_definition)?.to_string(),
            },
            message: papermc_definition,
        })
    }
}

#[async_trait]
impl Insert for AppCreateContext<PaperMcServerDefinition> {
    async fn insert(&self, connection: &impl ConnectionTrait) -> anyhow::Result<()> {
        let minecraft_server_definition = required_def!(self.message.minecraft_server_definition)?;
        let server_definition = required_def!(minecraft_server_definition.server_definition)?;

        server_definition
            .build_model(self)?
            .insert(connection)
            .await?;

        minecraft_server_definition
            .build_model(self)?
            .insert(connection)
            .await?;

        self.message.build_model(self)?.insert(connection).await?;

        Ok(())
    }
}

impl<T> InsertModel<PaperMcServerModel, AppCreateContext<T>> for PaperMcServerDefinition
where
    T: prost::Message,
{
    fn build_model(&self, context: &AppCreateContext<T>) -> anyhow::Result<PaperMcServerModel> {
        let paper_mc_project = PaperMcProject::from_i32(self.project)
            .with_context(|| "invalid paper mc project provided")?;

        Ok(PaperMcServerModel {
            id: Set(context.application.id.to_string()),
            project: Set(paper_mc_project.as_str_name().to_string()),
            build: Set(0),
            build_update_off: Set(false),
        })
    }
}
