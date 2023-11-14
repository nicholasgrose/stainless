use async_trait::async_trait;
use std::fmt::Debug;

use prost::Message;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use entity::application::ActiveModel as ApplicationModel;
use iron_api::ServerDefinition;

use crate::database::insert::{Insert, InsertModel};
use crate::manager::app::create::AppCreationSettings;

pub mod minecraft;

#[derive(Debug)]
pub struct AppCreateContext<M>
where
    M: Message,
{
    pub application: AppCreationSettings,
    pub message: M,
}

fn to_tonic_status(err: anyhow::Error) -> tonic::Status {
    tonic::Status::from_error(err.into())
}

#[async_trait]
impl<M> Insert<AppCreateContext<M>> for ServerDefinition
where
    M: prost::Message,
{
    async fn insert(
        &self,
        connection: &impl ConnectionTrait,
        context: &AppCreateContext<M>,
    ) -> anyhow::Result<()> {
        self.build_model(context).await?.insert(connection).await?;

        context
            .application
            .properties
            .command
            .insert(connection, &context.application.properties)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl<M> InsertModel<ApplicationModel, AppCreateContext<M>> for ServerDefinition
where
    M: prost::Message,
{
    async fn build_model(&self, context: &AppCreateContext<M>) -> anyhow::Result<ApplicationModel> {
        Ok(ApplicationModel {
            id: Set(context.application.properties.id.to_string()),
            name: Set(self.name.clone()),
            command: Set(context
                .application
                .properties
                .command
                .program
                .id
                .to_string()),
            active: Set(self.active),
        })
    }
}
