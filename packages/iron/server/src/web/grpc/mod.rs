use std::fmt::Debug;

use prost::Message;
use sea_orm::Set;

use entity::application::ActiveModel as ApplicationModel;
use iron_api::ServerDefinition;

use crate::database::insert::InsertModel;
use crate::manager::app::ApplicationSettings;

pub mod minecraft;

#[derive(Debug)]
pub struct AppCreateContext<M>
where
    M: Message,
{
    pub application: ApplicationSettings,
    pub message: M,
}

fn to_tonic_status(err: anyhow::Error) -> tonic::Status {
    tonic::Status::from_error(err.into())
}

impl<M> InsertModel<ApplicationModel, AppCreateContext<M>> for ServerDefinition
where
    M: prost::Message,
{
    fn build_model(&self, context: &AppCreateContext<M>) -> anyhow::Result<ApplicationModel> {
        Ok(ApplicationModel {
            id: Set(context.application.id.to_string()),
            name: Set(self.name.clone()),
            command: Set(context.application.command.clone()),
            active: Set(self.active),
        })
    }
}
