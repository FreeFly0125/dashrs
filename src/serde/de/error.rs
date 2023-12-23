//! Module containing the error type for deserialization errors

use std::fmt::Display;

use thiserror::Error;

/// Errors that can occur during deserialization
#[derive(Debug, Error)]
pub enum Error<'de> {
    /// End of input data was reached.
    ///
    /// This means that more data was expected to finish deserialization, however all input had
    /// already been processed
    #[error("Unexpected EOF while parsing")]
    Eof,

    /// Some custom error happened while deserializing the data at the specified index.
    #[error("{value:?} at index {index:?} caused {message}")]
    Custom {
        /// The error message
        message: String,

        /// The index of the data that caused the error
        ///
        /// Is [`None`] if the error happens at a point where no index was available, such as when
        /// parsing the index itself
        index: Option<&'de str>,

        /// The value that caused the error
        ///
        /// Not available if the error is not related to any value (for instance if the format
        /// itself was malformed).
        value: Option<&'de str>,
    },

    /// A given [`Deserializer`](serde::Deserializer) function was not supported
    #[error("unsupported deserializer function: {0}")]
    Unsupported(&'static str),
}

impl serde::de::Error for Error<'_> {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom {
            message: msg.to_string(),
            index: None,
            value: None,
        }
    }
}
