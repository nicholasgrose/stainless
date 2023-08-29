use std::fmt::Debug;

use prost::Message;

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
