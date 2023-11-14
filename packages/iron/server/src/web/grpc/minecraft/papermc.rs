use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

use entity::paper_mc_server::ActiveModel as PaperMcServerModel;
use iron_api::minecraft_service::{PaperMcProject, PaperMcServerDefinition};

use crate::database::insert::{Insert, InsertModel};
use crate::manager::app::create::AppCreationSettings;
use crate::manager::app::{AppEventHandlers, AppProperties};
use crate::manager::handlers::minecraft::papermc::PaperMcHandler;
use crate::web::grpc::minecraft::aikars_flags::AikarsFlags;
use crate::web::grpc::AppCreateContext;

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
                    command: AikarsFlags::try_from(minecraft_server_definition)?.into(),
                },
                handlers: AppEventHandlers {
                    async_handlers: vec![Arc::<PaperMcHandler>::default()],
                    sync_handlers: vec![],
                },
            },
            message: papermc_definition,
        })
    }
}

#[async_trait]
impl<C> Insert<C> for AppCreateContext<PaperMcServerDefinition>
where
    C: Debug,
{
    async fn insert(&self, connection: &impl ConnectionTrait, _context: &C) -> anyhow::Result<()> {
        let minecraft_server_definition = required_def!(self.message.minecraft_server_definition)?;
        let server_definition = required_def!(minecraft_server_definition.server_definition)?;

        server_definition.insert(connection, self).await?;

        minecraft_server_definition
            .build_model(self)
            .await?
            .insert(connection)
            .await?;

        self.message
            .build_model(self)
            .await?
            .insert(connection)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl<M> InsertModel<PaperMcServerModel, AppCreateContext<M>> for PaperMcServerDefinition
where
    M: prost::Message,
{
    async fn build_model(
        &self,
        context: &AppCreateContext<M>,
    ) -> anyhow::Result<PaperMcServerModel> {
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
