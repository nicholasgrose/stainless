use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

use entity::paper_mc_server::ActiveModel as PaperMcServerModel;
use iron_api::minecraft_service::{PaperMcProject, PaperMcServerDefinition};

use crate::database::insert::{Insert, InsertModel};
use crate::manager::app::events::{AppEvent, AppEventDispatcher};
use crate::manager::app::{AppCreationSettings, AppProperties};
use crate::web::grpc::minecraft::aikars_flags::AikarsFlags;
use crate::web::grpc::AppCreateContext;

#[derive(Default, Debug)]
pub struct PaperMcDispatcher;

#[async_trait]
impl AppEventDispatcher for PaperMcDispatcher {
    async fn dispatch(&self, _event: Arc<AppEvent>) -> anyhow::Result<()> {
        Ok(())
    }

    fn dispatch_sync(&self, _event: Arc<AppEvent>) -> anyhow::Result<()> {
        Ok(())
    }
}

impl AsRef<Self> for PaperMcDispatcher {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl TryFrom<PaperMcServerDefinition> for AppCreateContext<PaperMcServerDefinition> {
    type Error = anyhow::Error;

    fn try_from(papermc_definition: PaperMcServerDefinition) -> Result<Self, Self::Error> {
        let minecraft_server_definition =
            required_def!(papermc_definition.minecraft_server_definition)?;
        let server_definition = required_def!(minecraft_server_definition.server_definition)?;

        Ok(AppCreateContext {
            application: AppCreationSettings {
                properties: AppProperties {
                    id: Uuid::new_v4(),
                    name: server_definition.name.clone(),
                    command: AikarsFlags::try_from(minecraft_server_definition)?.to_string(),
                },
                starting_handlers: vec![Arc::<PaperMcDispatcher>::default()],
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

impl<M> InsertModel<PaperMcServerModel, AppCreateContext<M>> for PaperMcServerDefinition
where
    M: prost::Message,
{
    fn build_model(&self, context: &AppCreateContext<M>) -> anyhow::Result<PaperMcServerModel> {
        let paper_mc_project =
            PaperMcProject::try_from(self.project).context("invalid paper mc project provided")?;

        Ok(PaperMcServerModel {
            id: Set(context.application.properties.id.to_string()),
            project: Set(paper_mc_project.as_str_name().to_string()),
            build: Set(0),
            build_update_off: Set(false),
        })
    }
}
