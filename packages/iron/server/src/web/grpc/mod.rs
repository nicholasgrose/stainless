use std::fmt::Debug;

use prost::Message;
use sea_orm::Set;

use entity::application::ActiveModel as ApplicationModel;
use iron_api::ServerDefinition;

use crate::database::insert::InsertModel;
use crate::manager::Application;

pub mod minecraft;

#[derive(Debug)]
pub struct AppCreateContext<T>
where
    T: Message,
{
    pub application: Application,
    pub message: T,
}

fn to_tonic_status(err: anyhow::Error) -> tonic::Status {
    tonic::Status::from_error(err.into())
}

impl<T> InsertModel<ApplicationModel, AppCreateContext<T>> for ServerDefinition
where
    T: prost::Message,
{
    fn build_model(&self, context: &AppCreateContext<T>) -> anyhow::Result<ApplicationModel> {
        Ok(ApplicationModel {
            id: Set(context.application.id.to_string()),
            name: Set(self.name.clone()),
            command: Set(context.application.command.clone()),
            active: Set(self.active),
        })
    }
}
