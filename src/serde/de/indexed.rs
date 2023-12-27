//! Module containing the deserializer for robtop's indexed data format

use super::error::Error;
use serde::{
    de,
    de::{DeserializeSeed, Visitor},
    Deserializer,
};
use std::str::Split;

// Special versions of the trace and debug macros used in this module that are statically disabled
// in release mode. We do not want to explicitly pass "release_max_level_off" feature to log because
// we're in a library crate, and since features are additive, that would turn off release mode
// logging in every crate that depends on dash-rs.
macro_rules! trace {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        log::trace!($($t)*)
    };
}

macro_rules! debug {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        log::debug!($($t)*)
    };
}

/// Deserializer for RobTop's indexed data format
///
/// This format is used in server responses and when storing model.level data. It is based around
/// the idea to concatenate all fields of an object together, using a specific character sequence as
/// a separator.
///
/// There are two variants of this format:
///
/// * **Map-like**: Every second field is an key, which is almost always an integer. This key is
/// unique and tells us which field follows.
/// * **List-like**: There are no keys, identification of
/// fields has to occur based on the how many-th field they are. In this case the deserializer
/// generates artificial indices (which just count up by 1 for each field) for error messages.
#[derive(Debug)]
pub struct IndexedDeserializer<'de> {
    map_like: bool,
    splitter: Split<'de, &'de str>,
    input: &'de str,
    end_of_current_token: usize,
    delimiter: &'de str,
}

impl<'de> IndexedDeserializer<'de> {
    /// Constructs a new `IndexedDeserializer`
    ///
    /// # Arguments
    /// * *source*: The input string to deserialize
    /// * *delimiter*: The delimiter separating the individual fields
    /// * *map_like*: Whether the input is in map-like format or not (meaning it is in list-like
    ///   format)
    pub fn new(source: &'de str, delimiter: &'static str, map_like: bool) -> Self {
        trace!("Deserializing {} with delimiter '{}', maplike {}", source, delimiter, map_like);

        IndexedDeserializer {
            splitter: source.split(delimiter),
            map_like,
            input: source,
            end_of_current_token: source.as_ptr() as usize,
            delimiter,
        }
    }

    /// Returns the next token in the input string and consumes it.
    ///
    /// If the input string has already been fully consumed, returns [`Error::Eof`]. If the
    /// non-consumed part of the input starts with the delimiter, returns the empty string.
    /// Otherwise returns the sub-slice into the source representing the next token.
    fn consume_token(&mut self) -> Option<&'de str> {
        let tok = self.splitter.next()?;
        self.end_of_current_token = tok.as_ptr() as usize + tok.len();

        trace!("Splitting off token {}, remaining input: {}", tok, &self.input[self.position()..]);

        Some(tok)
    }

    fn position(&self) -> usize {
        self.end_of_current_token - self.input.as_ptr() as usize
    }

    fn nth_last(&self, nth: usize) -> Option<&'de str> {
        self.input[..self.position()].rsplit(self.delimiter).nth(nth - 1)
    }

    fn is_next_empty(&self) -> bool {
        &self.input[self.position() + self.delimiter.len()..self.position() + 2 * self.delimiter.len()] == self.delimiter
    }

    fn is_eof(&self) -> bool {
        self.input.len() <= self.position()
    }
}

macro_rules! delegate_to_from_str {
    ($deserialize_method:ident, $visitor_method:ident) => {
        fn $deserialize_method<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
        where
            V: Visitor<'de>,
        {
            let token = self.consume_token();

            trace!(
                "RobtopDeserializer::{} called called on {:?}",
                stringify!($deserialize_method),
                token
            );

            let token = token.ok_or(Error::Eof)?;

            match token.parse() {
                Ok(parsed) => visitor.$visitor_method(parsed),
                Err(error) => Err(Error::Custom {
                    message: error.to_string(),
                    index: None,
                    value: Some(token),
                }),
            }
        }
    };
}

impl<'a, 'de> Deserializer<'de> for &'a mut IndexedDeserializer<'de> {
    type Error = Error<'de>;

    delegate_to_from_str!(deserialize_i8, visit_i8);

    delegate_to_from_str!(deserialize_i16, visit_i16);

    delegate_to_from_str!(deserialize_i32, visit_i32);

    delegate_to_from_str!(deserialize_i64, visit_i64);

    delegate_to_from_str!(deserialize_u8, visit_u8);

    delegate_to_from_str!(deserialize_u16, visit_u16);

    delegate_to_from_str!(deserialize_u32, visit_u32);

    delegate_to_from_str!(deserialize_u64, visit_u64);

    delegate_to_from_str!(deserialize_f32, visit_f32);

    delegate_to_from_str!(deserialize_f64, visit_f64);

    fn deserialize_any<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        // the data format is by no means self describing
        Err(Error::Unsupported("deserialize_any"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        let token = self.consume_token();

        trace!("RobtopDeserializer::deserialize_bool called on {:?}", token);

        // Alright so robtop's encoding of boolean is the most inconsistent shit ever. The possible values
        // for `false` are "0" or the empty string. The possible values for `true` are 1, 2 or 10. While
        // this is no problem for serialization, the deserializer has no way of knowing what kinda of
        // boolean is being used and defaults to "0" for `false` and "1" for `true`. If some field deviates
        // from that, use a custom `deserialize_with`. Thanks.

        match token {
            Some("0") | Some("") | None => visitor.visit_bool(false),
            Some("1") | Some("2") | Some("10") => visitor.visit_bool(true),
            Some(value) => Err(Error::Custom {
                message: "Expected 0, 1, 2, 10 or the empty string".to_owned(),
                index: None,
                value: Some(value),
            }),
        }
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_char"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        let token = self.consume_token();

        trace!("RobtopDeserializer::deserialize_str called on {:?}", token);

        visitor.visit_borrowed_str(token.ok_or(Error::Eof)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        let token = self.consume_token();

        trace!("RobtopDeserializer::deserialize_string called on {:?}", token);

        visitor.visit_borrowed_str(token.ok_or(Error::Eof)?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_bytes"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_byte_buf"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        if self.is_eof() || self.is_next_empty() {
            trace!("RobtopDeserializer::deserialize_option called on empty string or EOF");

            let _ = self.consume_token(); // potentially skip the empty string. Explicitly ignore the return value in case we have Error::Eof

            visitor.visit_none()
        } else {
            trace!("RobtopDeserializer::deserialize_option called 'Some(_)')");

            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_unit"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_unit_struct"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_newtype_struct"))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            index: 0,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_tuple"))
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_tuple_struct"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapAccess { deserializer: self })
    }

    fn deserialize_struct<V>(
        self, _name: &'static str, _fields: &'static [&'static str], visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        if self.map_like {
            visitor.visit_map(MapAccess { deserializer: self })
        } else {
            self.deserialize_seq(visitor)
        }
    }

    fn deserialize_enum<V>(
        self, _name: &'static str, _variants: &'static [&'static str], _visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("deserialize_enum"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        trace!("RobtopDeserializer::deserialize_identifier called");

        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        // We are still very much not self describing, however we do need to correctly handle unimplemented
        // indices. By the time this is called, they key itself will already have been popped in our
        // `MapAccess` implementation. This means we need to skip exactly one item! We'll feed a `None` to
        // the visitor. Because idk what we really wanna do here otherwise
        let _token = self.consume_token();

        debug!(
            "Ignored token {:?}. Preceding token (potentially an unmapped index) was {:?}",
            _token,
            self.nth_last(1)
        );

        visitor.visit_none()
    }
}

const INDICES: [&str; 50] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24",
    "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46",
    "47", "48", "49", "50",
];

struct SeqAccess<'a, 'de> {
    deserializer: &'a mut IndexedDeserializer<'de>,
    index: usize,
}

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error<'de>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error<'de>>
    where
        T: DeserializeSeed<'de>,
    {
        self.index += 1;

        trace!("Deserializing list entry at index {}", self.index);

        match seed.deserialize(&mut *self.deserializer) {
            Err(Error::Eof) => Ok(None),
            Err(Error::Custom { message, value, .. }) => Err(Error::Custom {
                message,
                value: value.or_else(|| self.deserializer.nth_last(1)),
                index: Some(INDICES.get(self.index - 1).unwrap_or(&">=51")),
            }),
            Err(err) => Err(err),
            Ok(item) => Ok(Some(item)),
        }
    }
}

struct MapAccess<'a, 'de> {
    deserializer: &'a mut IndexedDeserializer<'de>,
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error<'de>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error<'de>>
    where
        K: DeserializeSeed<'de>,
    {
        trace!("Processing a map key");

        match seed.deserialize(&mut *self.deserializer) {
            Err(Error::Eof) => Ok(None),
            Err(Error::Custom { message, .. }) => Err(Error::Custom {
                message,
                value: None,
                index: self.deserializer.nth_last(1),
            }),
            Err(err) => Err(err),
            Ok(item) => Ok(Some(item)),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error<'de>>
    where
        V: DeserializeSeed<'de>,
    {
        trace!("Processing a map value",);

        match seed.deserialize(&mut *self.deserializer) {
            Err(Error::Custom { message, value, .. }) => Err(Error::Custom {
                message,
                value: value.or_else(|| self.deserializer.nth_last(1)),
                index: self.deserializer.nth_last(2),
            }),
            r => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::IndexedDeserializer;
    use serde::de::Deserialize;
    use std::collections::HashMap;

    const INPUT: &str = "1:hello:2:world";

    #[test]
    fn test_deserialize_map_like_to_hashmap() {
        // Illustrates how to deserialize some arbitrary RobTop string into a HashMap, for easier analysis.
        let mut deserializer = IndexedDeserializer::new(INPUT, ":", true);

        let map = HashMap::<&str, &str>::deserialize(&mut deserializer).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"hello"));
        assert_eq!(map.get("2"), Some(&"world"));
    }

    #[test]
    fn test_deserialize_map_like_last_empty() {
        // Illustrates how to deserialize some arbitrary RobTop string into a HashMap, for easier analysis.
        let mut deserializer = IndexedDeserializer::new("1:hello:2:", ":", true);

        let map = HashMap::<&str, &str>::deserialize(&mut deserializer).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"hello"));
        assert_eq!(map.get("2"), Some(&""));
    }

    #[test]
    fn test_deserialize_to_vec() {
        let mut deserializer = IndexedDeserializer::new(INPUT, ":", false);

        let vec = Vec::<&str>::deserialize(&mut deserializer).unwrap();

        assert_eq!(vec, INPUT.split(':').collect::<Vec<_>>())
    }
}
