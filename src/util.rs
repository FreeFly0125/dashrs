//! Module containing various utility functions related to processing Geometry Dash data

use serde::Serializer;

/// Performs RobTop's XOR en-/decoding routine on `encoded` using `key`
///
/// Note that although both `encrypted` and `key` are `str`s, the decryption
/// is done directly on the bytes, and the result of each byte-wise XOR
/// operation is casted to `char`, meaning this function only works for
/// ASCII strings.
#[inline]
pub fn cyclic_xor<T>(encoded: &mut [u8], key: &T)
where
    T: AsRef<[u8]> + ?Sized, // ?Sized needed here because we want for example to accept &[u8], where T would be [u8]
{
    // for_each usually specializes better for iterators
    // Also changed into using ^= for simplicity
    encoded.iter_mut().zip(key.as_ref().iter().cycle()).for_each(|(d, k)| *d ^= k);
}

pub fn option_variant_eq<A, B>(a: &Option<A>, b: &Option<B>) -> bool
where
    A: PartialEq<B>,
{
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, None) => true,
        _ => false,
    }
}

pub(crate) mod default_to_none {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Default + Serialize,
    {
        match to_serialize {
            None => T::default().serialize(serializer),
            Some(ref t) => t.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Default + Deserialize<'de> + PartialEq,
    {
        let deserialized = T::deserialize(deserializer)?;

        if deserialized == T::default() {
            Ok(None)
        } else {
            Ok(Some(deserialized))
        }
    }
}

pub(crate) fn false_to_empty_string<S: Serializer>(b: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    match *b {
        true => serializer.serialize_str("1"),
        false => serializer.serialize_str(""),
    }
}

pub(crate) fn true_to_two<S: Serializer>(b: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    match *b {
        true => serializer.serialize_str("2"),
        false => serializer.serialize_str("0"),
    }
}

pub(crate) fn true_to_ten<S: Serializer>(b: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    match *b {
        true => serializer.serialize_str("10"),
        false => serializer.serialize_str("0"),
    }
}

#[macro_export]
macro_rules! into_conversion {
    ($for:ty, $proxy_type:ty) => {
        impl $crate::serde::InternalProxy for $for {
            type DeserializeProxy = $proxy_type;
            type SerializeProxy<'a> = $proxy_type where Self: 'a;

            fn to_serialize_proxy(&self) -> $proxy_type {
                (*self).into()
            }

            fn from_deserialize_proxy(from: $proxy_type) -> $for {
                <$for>::from(from)
            }
        }
    };
}

#[macro_export]
macro_rules! dash_rs_newtype {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name<'a>(pub Cow<'a, str>);

        impl<'a> $crate::serde::InternalProxy for $name<'a> {
            type DeserializeProxy = &'a str;
            type SerializeProxy<'b> = &'b str where Self: 'b;

            fn to_serialize_proxy(&self) -> &str {
                use std::borrow::Borrow;

                self.0.borrow()
            }

            fn from_deserialize_proxy(from: &'a str) -> $name<'a> {
                $name(Cow::Borrowed(from))
            }
        }
    };
}
