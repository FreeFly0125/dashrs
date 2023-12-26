#![forbid(unsafe_code)]

pub mod model;
pub mod request;
pub mod response;
pub(crate) mod serde;
pub mod util;

pub use crate::serde::{Dash, DeError, GJFormat, IndexedDeserializer, IndexedSerializer, ProcessError, SerError, Thunk, ThunkProcessor};
