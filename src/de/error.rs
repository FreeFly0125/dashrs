//! Module containing the error type for deserialization errors

use serde::export::Formatter;
use std::fmt::Display;

/// Errors that can occur during deserialization
#[derive(Debug)]
pub enum Error<'de> {
    /// End of input data was reached.
    ///
    /// This means that more data was expected to finish deserialization, however all input had
    /// already been processed
    Eof,

    /// Some custom error happened during deserialization.
    ///
    /// Note that if an error occurs while processing an index, this variant is returned.
    Custom(String),

    /// Some custom error happened while deserializing the data at the specified index.
    CustomAt {
        /// The error message
        message: String,

        /// The index of the data that caused the error
        index: &'de str,
    },

    /// A given [`Deserializer`] function was not supported
    Unsupported(&'static str),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(s) => write!(f, "{}", s),
            Error::CustomAt { message, index } => write!(f, "{} at index '{}'", message, index),
            Error::Eof => write!(f, "Unexpected EOF while parsing"),
            Error::Unsupported(what) => write!(f, "unsupported deserializer function: {}", what),
        }
    }
}

impl std::error::Error for Error<'_> {}

impl serde::de::Error for Error<'_> {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}
