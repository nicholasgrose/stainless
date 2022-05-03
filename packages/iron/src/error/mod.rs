use anyhow::anyhow;
use std::{fmt::Debug, io::ErrorKind};

use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IronError {
    #[error("io error")]
    IO(std::io::Error),
    #[error("io error")]
    Web(actix_web::Error),
    #[error("general error")]
    Any(anyhow::Error),
}

impl ResponseError for IronError {}

impl From<IronError> for std::io::Error {
    fn from(val: IronError) -> Self {
        match val {
            IronError::IO(error) => error,
            IronError::Web(error) => {
                std::io::Error::new(ErrorKind::Other, anyhow!("response error: {}", error))
            }
            IronError::Any(error) => std::io::Error::new(ErrorKind::Other, error),
        }
    }
}

impl From<anyhow::Error> for IronError {
    fn from(error: anyhow::Error) -> IronError {
        IronError::Any(error)
    }
}

impl From<std::io::Error> for IronError {
    fn from(error: std::io::Error) -> IronError {
        IronError::IO(error)
    }
}

impl From<actix_web::Error> for IronError {
    fn from(error: actix_web::Error) -> IronError {
        IronError::Web(error)
    }
}
