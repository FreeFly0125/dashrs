mod de;
mod ser;
mod thunk;

pub use de::{error::Error as DeError, indexed::IndexedDeserializer};
pub use ser::{error::Error as SerError, indexed::IndexedSerializer, request::RequestSerializer};
use serde::{Deserializer, Serializer};
pub use thunk::{Base64Decoder, PercentDecoder, ProcessError, Thunk, ThunkProcessor};

use std::{borrow::Cow, io::Write};

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

pub trait Dash<'de>: Sized {
    fn dash_deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
    fn dash_serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

pub trait GJFormat<'de>: Dash<'de> {
    const DELIMITER: &'static str;
    const MAP_LIKE: bool;

    fn from_gj_str(input: &'de str) -> Result<Self, de::error::Error> {
        let mut indexed_deserializer = IndexedDeserializer::new(input, Self::DELIMITER, Self::MAP_LIKE);

        Self::dash_deserialize(&mut indexed_deserializer)
    }

    fn write_gj<W: Write>(&self, writer: W) -> Result<(), ser::error::Error> {
        let mut indexed_serializer = IndexedSerializer::new(Self::DELIMITER, writer, Self::MAP_LIKE);

        self.dash_serialize(&mut indexed_serializer)
    }
}

/// Trait describing an intermediate step between the raw Geomtry Dash data format, and the APIs exposed by dash-rs
///
/// Each field in a dash-rs struct that is mapped 1:1 to some index in the Geometry Dash data format is first deserialized to an intermediate proxy type (such as &'de str), before being converted
/// into the type it has in the public API (such as Thunk<'a, PercentDecoder>). This trait handles this conversion and its reciprocal.
pub trait InternalProxy {
    /// The type to which an index get deserialized in the internal representation
    type DeserializeProxy;
    /// The type
    type SerializeProxy<'a>
    where
        Self: 'a;

    fn to_serialize_proxy(&self) -> Self::SerializeProxy<'_>;
    fn from_deserialize_proxy(from: Self::DeserializeProxy) -> Self;
}

macro_rules! identity_conversion {
    ($($t: ty),*) => {
        $(
            impl $crate::serde::InternalProxy for $t {
                type DeserializeProxy = $t;
                type SerializeProxy<'a> = $t;

                fn to_serialize_proxy(&self) -> Self::SerializeProxy<'_> {
                    *self
                }

                fn from_deserialize_proxy(from: Self::DeserializeProxy) -> Self {
                    from
                }
            }
        )*
    };
}

identity_conversion!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, bool);

impl<'b, T: ToOwned + ?Sized + 'static> InternalProxy for Cow<'b, T> {
    type DeserializeProxy = &'b T;
    type SerializeProxy<'a> = &'a T where Self: 'a;

    fn to_serialize_proxy(&self) -> Self::SerializeProxy<'_> {
        self.as_ref()
    }

    fn from_deserialize_proxy(from: Self::DeserializeProxy) -> Self {
        Cow::Borrowed(from)
    }
}

impl<'b, T: ThunkProcessor> InternalProxy for Thunk<'b, T> {
    type DeserializeProxy = &'b str;
    type SerializeProxy<'a> = Cow<'a, str> where Self: 'a;

    fn to_serialize_proxy(&self) -> Self::SerializeProxy<'_> {
        self.as_unprocessed().unwrap()
    }

    fn from_deserialize_proxy(from: Self::DeserializeProxy) -> Self {
        Thunk::Unprocessed(Cow::Borrowed(from))
    }
}

impl<T: InternalProxy> InternalProxy for Option<T> {
    type DeserializeProxy = Option<T::DeserializeProxy>;
    type SerializeProxy<'a> = Option<T::SerializeProxy<'a>> where Self: 'a;

    fn to_serialize_proxy(&self) -> Self::SerializeProxy<'_> {
        self.as_ref().map(|t| t.to_serialize_proxy())
    }

    fn from_deserialize_proxy(from: Self::DeserializeProxy) -> Self {
        from.map(|f| T::from_deserialize_proxy(f))
    }
}
