use std::{fmt::Debug, io::ErrorKind};

use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IronError {
    #[error("io error: {source}")]
    IO {
        #[from]
        #[backtrace]
        source: std::io::Error,
    },
    #[error(transparent)]
    Other {
        #[from]
        #[backtrace]
        source: anyhow::Error,
    },
}

impl ResponseError for IronError {}

impl From<IronError> for std::io::Error {
    fn from(val: IronError) -> Self {
        match val {
            IronError::IO { source } => source,
            IronError::Other { source: _ } => std::io::Error::new(ErrorKind::Other, val),
        }
    }
}
