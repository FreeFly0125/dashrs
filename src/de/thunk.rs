use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, convert::TryFrom, str::Utf8Error};

/// Input value whose further deserialization has been delayed
///
/// This is often used if further processing would require an allocation (for instance when using
/// base64 decoding) or be very long (for instance parsing model.level data).
///
/// The required further processing should happen in the [`TryFrom`]
/// implementation and is invoked by calling [`Thunk::process`]. Think of it as [`Cow`] with extra
/// steps and potential new allocations instead of cloning.
///
/// The one requirement we impose on `P` is that it must implement [`Serialize`] and serialize to
/// exactly the unprocessed string it was constructed from (unless it was manually changed of
/// course, in which case it needs to correctly serialize to a string representation from which it
/// can be reconstructed via [`TryFrom`])
#[derive(Debug, Eq, PartialEq)]
pub enum Thunk<'a, P: TryFrom<&'a str> + Serialize> {
    /// Unprocessed value
    Unprocessed(&'a str),

    /// Processed value
    Processed(P),
}

impl<'a, P: TryFrom<&'a str> + Serialize> Serialize for Thunk<'a, P> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Thunk::Unprocessed(s) => serializer.serialize_str(s),
            Thunk::Processed(processed) => processed.serialize(serializer),
        }
    }
}

impl<'a, 'de: 'a, P: TryFrom<&'a str> + Serialize> Deserialize<'de> for Thunk<'a, P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).map(Thunk::Unprocessed)
    }
}

impl<'a, P: TryFrom<&'a str> + Serialize> Thunk<'a, P> {
    /// If this is a [`Thunk::Unprocessed`] variant, invokes the [`TryFrom`] impl and returns
    /// [`Thunk::Processed`]. Simply returns itself if this is a [`Thunk::Processed`] variant
    pub fn process(&mut self) -> Result<&P, P::Error> {
        if let Thunk::Unprocessed(raw_data) = self {
            *self = Thunk::Processed(P::try_from(raw_data)?)
        }

        match self {
            Thunk::Processed(p) => Ok(p),
            _ => unreachable!(),
        }
    }

    /// Returns the result of processing this [`Thunk`]
    pub fn into_processed(self) -> Result<P, P::Error> {
        match self {
            Thunk::Unprocessed(unprocessed) => P::try_from(unprocessed),
            Thunk::Processed(p) => Ok(p),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PercentDecoded<'a>(pub Cow<'a, str>);

impl<'a> TryFrom<&'a str> for PercentDecoded<'a> {
    type Error = Utf8Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        percent_decode_str(value).decode_utf8().map(PercentDecoded)
    }
}

impl<'a> Serialize for PercentDecoded<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let moo: Cow<str> = utf8_percent_encode(&*self.0, NON_ALPHANUMERIC).into();

        serializer.serialize_str(&*moo)
    }
}
