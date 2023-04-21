mod de;
mod ser;
mod thunk;

pub use de::{error::Error as DeError, indexed::IndexedDeserializer};
pub use ser::{error::Error as SerError, indexed::IndexedSerializer, request::RequestSerializer};
pub use thunk::{ProcessError, ThunkProcessor, Thunk, Base64Decoder, PercentDecoder};

use std::io::Write;

/// Trait implemented by objects that can be (de)serialized from/to RobTop's data formats
///
/// Internally, all types implementing supporting (de)serialization using RobTop's data formats do
/// so on a custom, private struct. This is mainly so that details from robtop's data
/// formats don't leak into dash-rs' data representation (as this allows the types in the public API
/// to implemente/derive Serialize/Deserialize in a way differing from the serialization required
/// for Robtop's data formats)
///
/// The lifetime `'a` is the lifetime of the borrowed data (either from the deserializer or from a
/// Cow that's been processed) an implementing struct contains.
pub trait HasRobtopFormat<'a>: Sized {
    /// Constructs [`Self`] from the given string in robtop-format borrowing/thunking all data.
    ///
    /// # Contract:
    /// This method performs no allocations. Unprocessed data should be wrapped inside a [`Thunk`]
    /// to be processed on-demand later, the remaining data should stay in
    /// [`Cow`](std::borrow::Cow)s
    fn from_robtop_str(input: &'a str) -> Result<Self, DeError>;

    /// Serializes [`self`] onto the given writer, in robtop format.
    fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError>;

    /// Serializes [`self`] to a string, in robtop format.
    fn to_robtop_string(&self) -> Result<String, SerError> {
        let mut buf: Vec<u8> = Vec::with_capacity(64);

        self.write_robtop_data(&mut buf)?;

        Ok(String::from_utf8(buf)?)
    }
}
