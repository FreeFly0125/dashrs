//! Module containing the error type for deserialization errors

use std::fmt::{Display, Formatter};

/// Errors that can occur during deserialization
#[derive(Debug)]
pub enum Error<'de> {
    /// End of input data was reached.
    ///
    /// This means that more data was expected to finish deserialization, however all input had
    /// already been processed
    Eof,

    /// Some custom error happened while deserializing the data at the specified index.
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
    Unsupported(&'static str),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom { message, index, value } => write!(f, "{:?} at index {:?} caused {}", value, index, message),
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
        Error::Custom {
            message: msg.to_string(),
            index: None,
            value: None,
        }
    }
}
