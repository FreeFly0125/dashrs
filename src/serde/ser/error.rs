use std::{
    fmt::{Display, Formatter},
    io,
};

/// Errors that can occur during serialization
#[derive(Debug)]
pub enum Error {
    /// Some custom error happened during serialization.
    Custom(String),

    /// A given [`Serializer`](serde::Serializer) function was not supported
    Unsupported(&'static str),

    Io(io::Error),

    Utf8(std::string::FromUtf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::Unsupported(what) => write!(f, "unsupported serializer function: {}", what),
            Error::Io(err) => write!(f, "io error: {}", err),
            Error::Utf8(err) => write!(f, "failed utf8 conversion: {}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::Utf8(error)
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
