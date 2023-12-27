use base64::{engine::general_purpose::URL_SAFE, DecodeError, DecodeSliceError, Engine};
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{ser::Error as _, Deserialize, Serialize, Serializer};
use std::{
    borrow::{Borrow, Cow},
    mem,
    num::ParseIntError,
    str::Utf8Error,
    string::FromUtf8Error,
};
use thiserror::Error;

/// Enum modelling the different errors that can occur during processing of a [`Thunk`]
///
/// ## Why is this a seperate enum
/// One might wonder why this enum exists, and why we don't simply reuse
/// [`Error`](::serde::de::Error). The main reason is that I do not want to include variants
/// in that enum that do not occur during the actual deserialization phase. The second reason has to
/// do with lifetimes: Just using `Error<'a>` for the return type in the [`ThunkProcessor`] functions
/// used by [`Thunk`] is not possible. The reason for that is that processing errors are returned in
/// contexts where data is transformed into owned representations. This means we cannot simply reuse
/// the lifetime the input data is bound to for our errors, as the errors potentially have to
/// outlive the input data (in the worst case they have to be `'static`). Adding a new lifetime to
/// `Thunk` just to use that for the error type is obviously impractical, however it is possible to
/// use `Error<'static>`, which at least doesn't add more downsides. However it still leaves us with
/// an error enum dealing with too much stuff.
#[derive(Debug, Error)]
pub enum ProcessError {
    /// Some utf8 encoding error occurred during processing
    #[error("{0}")]
    Utf8(#[from] Utf8Error),

    /// Some utf8 encoding error occurred while processing after some backing storage was allocated
    #[error("{0}")]
    FromUtf8(#[from] FromUtf8Error),

    /// Some base64 decoding error occurred during processing
    #[error("{0}")]
    Base64(#[from] DecodeSliceError),

    /// Some error occurred when parsing a number
    #[error("{0}")]
    IntParse(#[from] ParseIntError),

    /// Incorrect number of items when parsing a comma separated list (e.g. if an RGB list only has
    /// two entries)
    #[error("Incorrect number of items in comma separated list. Expected {expected}")]
    IncorrectLength { expected: usize },

    #[error("Received value that cannot be represented in Geometry Dash data format")]
    Unrepresentable,
}

impl From<DecodeError> for ProcessError {
    fn from(value: DecodeError) -> Self {
        ProcessError::Base64(DecodeSliceError::DecodeError(value))
    }
}

/// Input value whose further deserialization has been delayed
///
/// This is often used if further processing would require an allocation (for instance when using
/// base64 decoding) or be very long (for instance parsing level data).
///
/// The required further processing should happen in the [`ThunkProcessor`] implementation, which is
/// invoked by calling [`Thunk::process`]. Think of it as [`Cow`] with extra steps and potential new
/// allocations instead of cloning.
#[derive(Debug, Eq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Thunk<'a, C: ThunkProcessor> {
    #[serde(skip)]
    Unprocessed(Cow<'a, str>),
    Processed(C::Output<'a>),
}

impl<'a, 'b, P: ThunkProcessor> PartialEq<Thunk<'b, P>> for Thunk<'a, P>
where
    P::Output<'a>: PartialEq<P::Output<'b>>,
{
    fn eq(&self, other: &Thunk<'b, P>) -> bool {
        match (self, other) {
            (Thunk::Processed(o1), Thunk::Processed(o2)) => o1 == o2,
            (Thunk::Unprocessed(s1), Thunk::Unprocessed(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl<'a, C: ThunkProcessor> Serialize for Thunk<'a, C>
where
    for<'b> C::Output<'b>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(Cow::Borrowed(unprocessed))
                .map_err(S::Error::custom)?
                .serialize(serializer),
            Thunk::Processed(processed) => processed.serialize(serializer),
        }
    }
}

/// Trait describing how thunks should process their data
///
/// This trait provides the means to translate from and into RobTop's representation for thunked
/// data, while not being used in the (de)serialization into any other data format.
pub trait ThunkProcessor {
    type Error: std::error::Error;
    type Output<'a>;

    /// Takes some data from the [`Thunk::Unprocessed`] variant and processes it
    ///
    /// This function is *not* called automatically during deserialization from a RobTop data
    /// format.
    fn from_unprocessed(unprocessed: Cow<str>) -> Result<Self::Output<'_>, Self::Error>;

    /// Takes some processed thunk value and converts it into RobTop-representation
    fn as_unprocessed<'b>(processed: &'b Self::Output<'_>) -> Result<Cow<'b, str>, Self::Error>;

    /// The presence of this function essentially forces [`ThunkProcessor::Output`] to be covariant
    /// in its lifetime.
    ///
    /// We need it to be covariant so that functions like [`Thunk::as_unprocessed`] can be implemented.
    /// Since the lifetime associated with the output is assumed to be originating from some `&'a str`,
    /// and should only describe subslices of said input string, enforcing covariance on this trait should
    /// cause no limitations in praxis (e.g. all implementations here should just be able to return `output`
    /// identically).
    ///
    /// We need this function due to a limitation of GATs, where for soundness reasons they have to be assumed
    /// to be invariant, yet the language provides no way for a trait to explicitly require different variance.
    fn downcast_output_lifetime<'b: 'c, 'c, 's>(output: &'s Self::Output<'b>) -> &'s Self::Output<'c>;
}

impl<'a, C: ThunkProcessor> Thunk<'a, C> {
    /// If this is a [`Thunk::Unprocessed`] variant, calls [`ThunkProcessor::from_unprocessed`] and
    /// returns [`Thunk::Processed`]. Simply returns `self` if this is a [`Thunk::Processed`]
    /// variant
    pub fn process(&mut self) -> Result<&mut C::Output<'a>, C::Error> {
        if let Thunk::Unprocessed(raw_data) = self {
            *self = Thunk::Processed(C::from_unprocessed(mem::take(raw_data))?)
        }

        match self {
            Thunk::Processed(p) => Ok(p),
            _ => unreachable!(),
        }
    }

    pub fn as_unprocessed(&self) -> Result<Cow<str>, C::Error> {
        match self {
            Thunk::Unprocessed(unprocessed) => Ok(Cow::Borrowed(unprocessed)),
            Thunk::Processed(content) => C::as_unprocessed(content),
        }
    }

    /// Returns the result of processing this [`Thunk`]
    pub fn into_processed(self) -> Result<C::Output<'a>, C::Error> {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(unprocessed),
            Thunk::Processed(p) => Ok(p),
        }
    }

    pub fn as_processed<'b>(&'b self) -> Result<Cow<'b, C::Output<'b>>, C::Error>
    where
        C::Output<'b>: Clone,
    {
        match self {
            Thunk::Unprocessed(unprocessed) => C::from_unprocessed(Cow::Borrowed(unprocessed.borrow())).map(Cow::Owned),
            Thunk::Processed(processed) => Ok(Cow::Borrowed(C::downcast_output_lifetime(processed))),
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct PercentDecoder;

impl ThunkProcessor for PercentDecoder {
    type Error = ProcessError;
    type Output<'a> = Cow<'a, str>;

    fn from_unprocessed(unprocessed: Cow<str>) -> Result<Self::Output<'_>, Self::Error> {
        match unprocessed {
            Cow::Borrowed(unprocessed) => percent_decode_str(unprocessed).decode_utf8().map_err(ProcessError::Utf8),
            Cow::Owned(unprocessed) => match percent_decode_str(&unprocessed).decode_utf8().map_err(ProcessError::Utf8)? {
                Cow::Owned(decoded) => Ok(Cow::Owned(decoded)),
                _ => Ok(Cow::Owned(unprocessed)),
            },
        }
    }

    fn as_unprocessed<'b>(processed: &'b Self::Output<'_>) -> Result<Cow<'b, str>, Self::Error> {
        Ok(utf8_percent_encode(processed.as_ref(), ROBTOP_SET).into())
    }

    fn downcast_output_lifetime<'b: 'c, 'c, 's>(output: &'s Self::Output<'b>) -> &'s Self::Output<'c> {
        output
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Base64Decoder;

impl ThunkProcessor for Base64Decoder {
    type Error = ProcessError;
    type Output<'a> = Cow<'a, str>;

    fn from_unprocessed(unprocessed: Cow<str>) -> Result<Self::Output<'_>, Self::Error> {
        let vec = URL_SAFE.decode(&*unprocessed)?;
        let string = String::from_utf8(vec).map_err(ProcessError::FromUtf8)?;

        Ok(Cow::Owned(string))
    }

    fn as_unprocessed<'b>(processed: &'b Self::Output<'_>) -> Result<Cow<'b, str>, Self::Error> {
        Ok(Cow::Owned(URL_SAFE.encode(&**processed)))
    }

    fn downcast_output_lifetime<'b: 'c, 'c, 's>(output: &'s Self::Output<'b>) -> &'s Self::Output<'c> {
        output
    }
}
