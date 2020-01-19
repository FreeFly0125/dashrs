use serde::export::Formatter;
use std::fmt::Display;

/// Errors that can occur during serialization
#[derive(Debug)]
pub enum Error {
    /// Some custom error happened during serialization.
    Custom(String),

    /// A given [`Serializer`] function was not supported
    Unsupported(&'static str)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::Unsupported(what) => write!(f, "unsupported serializer function: {}", what)
        }
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}
