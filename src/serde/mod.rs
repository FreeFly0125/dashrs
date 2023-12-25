mod de;
mod ser;
mod thunk;

pub use de::{error::Error as DeError, indexed::IndexedDeserializer};
pub use ser::{error::Error as SerError, indexed::IndexedSerializer, request::RequestSerializer};
use serde::{Deserializer, Serializer};
pub use thunk::{Base64Decoder, PercentDecoder, ProcessError, Thunk, ThunkProcessor};

use std::{borrow::Cow, io::Write};

/// Trait for objects that can be (de)serialized from some Geometry Dash data format (e.g. an
/// indexed description).
///
/// Internally, all types supporting (de)serialization using a Geometry Dash data format do so on
/// custom, private structs. This is mainly so that details from robtop's data
/// formats don't leak into dash-rs' data representation (as this allows the types in the public API
/// to implemente/derive Serialize/Deserialize in a way differing from the serialization required
/// for Robtop's data formats). Then, this internal representation is converted into the types
/// exposed by `dash-rs`' API using either infallible conversions, or [`Thunk`]ing
///
/// The lifetime `'de` is the same lifetime that [`serde`](serde.rs) uses on its
/// [`Deserialize`](serde::Deserialize) trait, see also [the serde documentation].
///
/// [1]: https://serde.rs/lifetimes.html
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

/// Trait describing an intermediate step between the raw Geomtry Dash data format, and the APIs
/// exposed by dash-rs
///
/// Each field in a dash-rs struct that is mapped 1:1 to some index in the Geometry Dash data format
/// is first deserialized to an intermediate proxy type (such as &'de str), before being converted
/// into the type it has in the public API (such as Thunk<'a, PercentDecoder>). This trait handles
/// this conversion and its reciprocal.
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
