use std::{fmt::Debug, io::ErrorKind};

use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IronError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for IronError {}

impl From<IronError> for std::io::Error {
    fn from(val: IronError) -> Self {
        match val {
            IronError::IO(error) => error,
            IronError::Other(error) => std::io::Error::new(ErrorKind::Other, error),
        }
    }
}
