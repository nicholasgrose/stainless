use std::{fmt::Debug, io::ErrorKind};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IronError {
    #[error("io error: {source}")]
    IO {
        #[from]
        #[backtrace]
        source: std::io::Error,
    },
    #[error("r2d2 error: {source}")]
    R2D2 {
        #[from]
        #[backtrace]
        source: r2d2::Error,
    },
    #[error("diesel error: {source}")]
    Diesel {
        #[from]
        #[backtrace]
        source: diesel::result::Error,
    },
    #[error(transparent)]
    Other {
        #[from]
        #[backtrace]
        source: anyhow::Error,
    },
}

impl From<IronError> for std::io::Error {
    fn from(val: IronError) -> Self {
        match val {
            IronError::IO { source } => source,
            IronError::R2D2 { source: _ } => std::io::Error::new(ErrorKind::Other, val),
            IronError::Diesel { source: _ } => std::io::Error::new(ErrorKind::Other, val),
            IronError::Other { source: _ } => std::io::Error::new(ErrorKind::Other, val),
        }
    }
}
