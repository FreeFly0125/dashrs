use base64::DecodeError;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{export::Formatter, Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, convert::TryFrom, fmt::Display, str::Utf8Error, num::ParseIntError};

/// Enum modelling the different errors that can occur during processing of a [`Thunk`]
///
/// ## Why is this a seperate enum
/// One might wonder why this enum exists, and why we don't simply reuse
/// [`Error`](::de::error::Error). The main reason is that I do not want to include variants in that
/// enum that do not occur during the actual deserialization phase. The second reason has to do with
/// lifetimes: Just using `Error<'a>` for the error type in the [`TryFrom`] impls used by `Thunk` is
/// not possible. The reason for that is that processing errors are returned in contexts where data
/// is transformed into owned representations. This means we cannot simply reuse the lifetime the
/// input data is bound to for our errors, as the errors potentially have to outlive the input data
/// (in the worst case they have to be `'static`). Adding a new lifetime to `Thunk` just to use that
/// for the error type is obviously impractical, however it is possible to use `Error<'static>`,
/// which at least doesn't add more downsides. However it still leaves us with an error enum dealing
/// with too much stuff.
#[derive(Debug)]
pub enum ProcessError {
    /// Some utf8 encoding error occurred during processing
    Utf8(Utf8Error),

    /// Some base64 decoding error occurred during processing
    Base64(DecodeError),

    // Some error occurred when parsing a number
    IntParse(ParseIntError),
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::Utf8(utf8) => utf8.fmt(f),
            ProcessError::Base64(decode) => decode.fmt(f),
            ProcessError::IntParse(int) => int.fmt(f),
        }
    }
}

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
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Thunk<'a, P: TryFrom<&'a str, Error = ProcessError> + Serialize> {
    /// Unprocessed value
    Unprocessed(&'a str),

    /// Processed value
    Processed(P),
}

impl<'a, P: TryFrom<&'a str, Error = ProcessError> + Serialize> Serialize for Thunk<'a, P> {
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

impl<'a, 'de: 'a, P: TryFrom<&'a str, Error = ProcessError> + Serialize> Deserialize<'de> for Thunk<'a, P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).map(Thunk::Unprocessed)
    }
}

impl<'a, P: TryFrom<&'a str, Error = ProcessError> + Serialize> Thunk<'a, P> {
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
/// Set of characters RobTop encodes when doing percent encoding
///
/// This is a subset of [`percent_encoding::NON_ALPHANUMERIC`], which encodes too many characters
pub const ROBTOP_SET: &AsciiSet = &CONTROLS
    .add(b' ')  // TODO: investigate if this is part of the set. Song links never contain spaces
    .add(b':')
    .add(b'/')
    .add(b'?')
    .add(b'~');

#[derive(Debug, Eq, PartialEq)]
pub struct PercentDecoded<'a>(pub Cow<'a, str>);

impl<'a> TryFrom<&'a str> for PercentDecoded<'a> {
    type Error = ProcessError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        percent_decode_str(value)
            .decode_utf8()
            .map(PercentDecoded)
            .map_err(ProcessError::Utf8)
    }
}

impl<'a> Serialize for PercentDecoded<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let moo: Cow<str> = utf8_percent_encode(&*self.0, ROBTOP_SET).into();

        serializer.serialize_str(&*moo)
    }
}
