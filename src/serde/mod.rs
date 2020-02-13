mod de;
mod ser;
mod thunk;

pub use de::{error::Error as DeError, indexed::IndexedDeserializer};
pub use ser::{error::Error as SerError, indexed::IndexedSerializer};
use serde::{Deserialize, Serialize};
pub(crate) use thunk::Internal;
pub use thunk::{PercentDecoded, ProcessError, Thunk};

/// Trait implemented by objects that can be (de)serialized from/to RobTop's data formats
///
/// Structs implementing this trait can be used with the [`from_robtop_str`]/[`to_robtop_data`]
/// functions.
///
/// In general, all types implementing supporting (de)serialization using RobTop's data formats do
/// so on a different struct, which should be set to this traits
/// [`Internal`](HasRobtopFormat::Internal) type. This is mainly so that details from robtop's data
/// formats don't leak into dash-rs' data representation.
///
/// The lifetime `'a` is the lifetime of the borrowed data (either from the deserializer or from a
/// Cow that's been processed) an implementing struct contains
pub trait HasRobtopFormat<'a> {
    /// The internal type used to (de)serialize this object into RobTop's data format
    type Internal: Deserialize<'a> + Serialize;

    /// The delimited used to separate fields of this object in RobTop's data format
    const DELIMITER: &'static str;

    /// Whether this object gets serialized "map-like" meaning that each field will receive an
    /// numerical index in RobTop's data format
    const MAP_LIKE: bool;

    fn into_internal(self) -> Self::Internal;
    fn from_internal(int: Self::Internal) -> Self;
}

// We use this trait to fake ourselves some high-kinder-lifetimes. More precisely, this trait allows
// us to perform the following conversion
// ==================================================================
// &'shorter Self<'longer> --> Self<'shorter> where 'longer: 'shorter
// ==================================================================
// The operation may only re-borrow data, and not perform any clones. This trait is effectively
// pub(crate)
pub trait ShortenLifetime<'shorter> {
    type Shortened;

    fn shorten(&'shorter self) -> Self::Shortened;
}

pub fn from_robtop_str<'a, T: HasRobtopFormat<'a>>(input: &'a str) -> Result<T, DeError> {
    let mut deserializer = IndexedDeserializer::new(input, T::DELIMITER, T::MAP_LIKE);

    let internal = T::Internal::deserialize(&mut deserializer)?;

    Ok(T::from_internal(internal))
}

pub fn to_robtop_data<'a, 'b, T>(t: &'b T) -> Result<Vec<u8>, SerError>
where
    T::Shortened: HasRobtopFormat<'b>,
    T: ShortenLifetime<'b>,
{
    let mut serializer = IndexedSerializer::new(T::Shortened::DELIMITER, T::Shortened::MAP_LIKE);
    let with_shorter_lifetime = t.shorten();

    with_shorter_lifetime.into_internal().serialize(&mut serializer)?;

    Ok(serializer.finish().into_bytes()) // FIXME: change the .finish() method
}
