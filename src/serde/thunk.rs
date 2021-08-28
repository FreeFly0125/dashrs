use base64::{DecodeError, URL_SAFE};
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, fmt::Display, num::ParseIntError, str::Utf8Error, string::FromUtf8Error};
use std::fmt::Formatter;

/// Enum modelling the different errors that can occur during processing of a [`Thunk`]
///
/// ## Why is this a seperate enum
/// One might wonder why this enum exists, and why we don't simply reuse
/// [`Error`](::serde::de::error::Error). The main reason is that I do not want to include variants
/// in that enum that do not occur during the actual deserialization phase. The second reason has to
/// do with lifetimes: Just using `Error<'a>` for the return type in the [`ThunkContent`] functions
/// used by [`Thunk`] is not possible. The reason for that is that processing errors are returned in
/// contexts where data is transformed into owned representations. This means we cannot simply reuse
/// the lifetime the input data is bound to for our errors, as the errors potentially have to
/// outlive the input data (in the worst case they have to be `'static`). Adding a new lifetime to
/// `Thunk` just to use that for the error type is obviously impractical, however it is possible to
/// use `Error<'static>`, which at least doesn't add more downsides. However it still leaves us with
/// an error enum dealing with too much stuff.
#[derive(Debug)]
pub enum ProcessError {
    /// Some utf8 encoding error occurred during processing
    Utf8(Utf8Error),

    /// Some utf8 encoding error occurred while processing after some backing storage was allocated
    FromUtf8(FromUtf8Error),

    /// Some base64 decoding error occurred during processing
    Base64(DecodeError),

    /// Some error occurred when parsing a number
    IntParse(ParseIntError),
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::Utf8(utf8) => utf8.fmt(f),
            ProcessError::Base64(decode) => decode.fmt(f),
            ProcessError::IntParse(int) => int.fmt(f),
            ProcessError::FromUtf8(from_utf8) => from_utf8.fmt(f),
        }
    }
}

impl std::error::Error for ProcessError {}

/// Input value whose further deserialization has been delayed
///
/// This is often used if further processing would require an allocation (for instance when using
/// base64 decoding) or be very long (for instance parsing level data).
///
/// The required further processing should happen in the [`ThunkContent`] implementation, which is
/// invoked by calling [`Thunk::process`]. Think of it as [`Cow`] with extra steps and potential new
/// allocations instead of cloning.
#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Thunk<'a, C: ThunkContent<'a>> {
    /// Unprocessed value
    #[serde(skip)]
    Unprocessed(&'a str),

    /// Processed value
    Processed(C),
}

/// Essentially the same as [`Thunk`], but with the difference that the `Processed` variant is
/// borrowed
///
/// This is used to serialize objects back into robtop's format.
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum RefThunk<'input, 'content, C: ThunkContent<'input>> {
    /// Unprocessed value
    Unprocessed(&'input str),

    /// Processed value
    Processed(&'content C),
}

impl<'a, C: ThunkContent<'a> + Serialize> Serialize for Thunk<'a, C> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(unprocessed).map_err(S::Error::custom)?.serialize(serializer),
            Thunk::Processed(processed) => processed.serialize(serializer),
        }
    }
}

impl<'input, 'content, C: ThunkContent<'input>> Serialize for RefThunk<'input, 'content, C> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            RefThunk::Unprocessed(unprocessed) => serializer.serialize_str(unprocessed),
            RefThunk::Processed(ref processed) =>
                match processed.as_unprocessed().map_err(serde::ser::Error::custom)? {
                    Cow::Borrowed(s) => serializer.serialize_str(s),
                    Cow::Owned(s) => s.serialize(serializer),
                },
        }
    }
}

impl<'de: 'input, 'input, 'content, C: ThunkContent<'input>> Deserialize<'de> for RefThunk<'input, 'content, C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).map(RefThunk::Unprocessed)
    }
}

/// Trait structs which are used in the [`Thunk::Processed`] variant have to implement.
///
/// This trait provides the means to translate from and into RobTop's representation for thunked
/// data, while not being used in the (de)serialization into any other data format.
pub trait ThunkContent<'a>: Sized {
    type Error: std::error::Error;

    /// Takes some data from the [`Thunk::Unprocessed`] variant and processes it
    ///
    /// This function is *not* called automatically during deserialization from a RobTop data
    /// format.
    fn from_unprocessed(unprocessed: &'a str) -> Result<Self, Self::Error>;

    /// Takes some processed thunk value and converts it into RobTop-representation
    fn as_unprocessed(&self) -> Result<Cow<str>, Self::Error>;
}

// effectively pub(crate) since it's not reexported in lib.rs
/// Marker type used to differentiate how [`Thunk`]s should serialize.
///
/// A `Internal<Thunk<C>>` should serialize its contents to RobTop's data format, while a `Thunk` in
/// general should serialize to a sane format.
#[derive(Debug)]
pub struct Internal<I>(pub(crate) I);

impl<'a, C: ThunkContent<'a>> Thunk<'a, C> {
    /// If this is a [`Thunk::Unprocessed`] variant, calls [`ThunkContent::from_unprocessed`] and
    /// returns [`Thunk::Processed`]. Simply returns `self` if this is a [`Thunk::Processed`]
    /// variant
    pub fn process(&mut self) -> Result<&C, C::Error> {
        if let Thunk::Unprocessed(raw_data) = self {
            *self = Thunk::Processed(C::from_unprocessed(raw_data)?)
        }

        match self {
            Thunk::Processed(p) => Ok(p),
            _ => unreachable!(),
        }
    }

    /// Returns the result of processing this [`Thunk`]
    pub fn into_processed(self) -> Result<C, C::Error> {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(unprocessed),
            Thunk::Processed(p) => Ok(p),
        }
    }

    // TODO: uhh maybe AsRef or Borrow or something would be the appropriate trait here?
    pub(crate) fn as_ref_thunk<'content>(&'content self) -> RefThunk<'a, 'content, C> {
        match self {
            Thunk::Unprocessed(raw) => RefThunk::Unprocessed(raw),
            Thunk::Processed(processed) => RefThunk::Processed(processed),
        }
    }
}
/// Set of characters RobTop encodes when doing percent encoding
///
/// This is a subset of [`percent_encoding::NON_ALPHANUMERIC`], since that encodes too many
/// characters
pub const ROBTOP_SET: &AsciiSet = &CONTROLS
    .add(b' ')  // TODO: investigate if this is part of the set. Song links never contain spaces
    .add(b':')
    .add(b'/')
    .add(b'?')
    .add(b'~');

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct PercentDecoded<'a>(#[serde(borrow)] pub Cow<'a, str>);

impl<'a> ThunkContent<'a> for PercentDecoded<'a> {
    type Error = ProcessError;

    fn from_unprocessed(unprocessed: &'a str) -> Result<Self, ProcessError> {
        percent_decode_str(unprocessed)
            .decode_utf8()
            .map(PercentDecoded)
            .map_err(ProcessError::Utf8)
    }

    fn as_unprocessed(&self) -> Result<Cow<str>, ProcessError> {
        Ok(utf8_percent_encode(self.0.as_ref(), ROBTOP_SET).into())
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Base64Decoded<'a>(pub Cow<'a, str>);

impl<'a> ThunkContent<'a> for Base64Decoded<'a> {
    type Error = ProcessError;

    fn from_unprocessed(unprocessed: &'a str) -> Result<Self, ProcessError> {
        let vec = base64::decode_config(unprocessed, URL_SAFE).map_err(ProcessError::Base64)?;
        let string = String::from_utf8(vec).map_err(ProcessError::FromUtf8)?;

        Ok(Base64Decoded(Cow::Owned(string)))
    }

    fn as_unprocessed(&self) -> Result<Cow<str>, ProcessError> {
        Ok(Cow::Owned(base64::encode_config(&*self.0, URL_SAFE)))
    }
}
