use std::{fmt::Display, io};

use thiserror::Error;

/// Errors that can occur during serialization
#[derive(Debug, Error)]
pub enum Error {
    /// Some custom error happened during serialization.
    #[error("{0}")]
    Custom(String),

    /// A given [`Serializer`](serde::Serializer) function was not supported
    #[error("unsupported serializer function: {0}")]
    Unsupported(&'static str),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("failed utf8 conversion: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}
