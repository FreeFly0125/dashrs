//! Module containing the deserializer for robtop's indexed data format

use super::error::Error;
use log::{trace, warn};
use serde::{
    de,
    de::{DeserializeSeed, Error as _, Visitor},
    Deserializer,
};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Delimiter {
    Char(char),
    String(&'static str),
}

impl Delimiter {
    fn len(&self) -> usize {
        match self {
            Delimiter::Char(_) => 1,
            Delimiter::String(s) => s.len(),
        }
    }
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
    source: &'de str,
    delimiter: Delimiter,
    map_like: bool,
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
        IndexedDeserializer {
            source,
            delimiter: if delimiter.len() == 1 {
                Delimiter::Char(delimiter.chars().nth(0).unwrap())
            } else {
                Delimiter::String(delimiter)
            },
            map_like,
        }
    }

    fn peek_item(&self) -> Result<Option<&'de str>, Error<'de>> {
        if self.source == "" {
            return Err(Error::Eof)
        }

        let index = match self.delimiter {
            Delimiter::Char(c) => self.source.find(c),
            Delimiter::String(s) => self.source.find(s),
        };

        Ok(match index {
            Some(index) if index == 0 => None,
            Some(index) => Some(&self.source[0..index]),
            None => Some(&self.source[..]),
        })
    }

    fn next_item(&mut self) -> Result<Option<&'de str>, Error<'de>> {
        let item = self.peek_item()?;

        // delimiter + length of potential content (0 is two consecutive delimiters)
        let split_off = self.delimiter.len() + item.map(|item| item.len()).unwrap_or_default();

        if split_off < self.source.len() {
            self.source = &self.source[split_off..]
        } else {
            self.source = ""
        }

        trace!("Dropped prefix from input, remaining is '{}'", self.source);

        Ok(item)
    }
}

macro_rules! delegate_to_from_str {
    ($deserialize_method: ident, $visitor_method: ident) => {
        fn $deserialize_method<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
        where
            V: Visitor<'de>
        {
            trace!("RobtopDeserializer::{} called called on {:?}", stringify!($deserialize_method), self.peek_item());

            match self.next_item()?.map(FromStr::from_str) {
                Some(Ok(parsed)) => visitor.$visitor_method(parsed),
                Some(Err(error)) => Err(Error::custom(error)),
                None => visitor.visit_none()
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
        unimplemented!("deserialize_any")
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        trace!("RobtopDeserializer::deserialize_bool called on {:?}", self.peek_item());

        // Alright so robtop's encoding of boolean is the most inconsistent shit ever. The most common case
        // is that '0' and the empty string mean false, while '1' means true. However, there is also the
        // rare variant where '1' means false as well and only '2' means true. If that is ever used, please
        // use a custom deserialization routine via 'deserialize_with'.

        match self.next_item() {
            Ok(None) | Err(Error::Eof) => visitor.visit_bool(false),
            Ok(Some("0")) => visitor.visit_bool(false),
            Ok(Some("1")) => visitor.visit_bool(true),
            Ok(Some(s)) => Err(Error::Custom(format!("Expected 0, 1 or the empty string, got '{}'", s))),
            Err(err) => Err(err),
        }
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_char")
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        trace!("RobtopDeserializer::deserialize_str called on {:?}", self.peek_item());

        match self.next_item()? {
            Some(string) => visitor.visit_borrowed_str(string),
            None => visitor.visit_none(),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        trace!("RobtopDeserializer::deserialize_string called on {:?}", self.peek_item());

        match self.next_item()? {
            Some(string) => visitor.visit_borrowed_str(string),
            None => visitor.visit_none(),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_bytes")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_byte_buf")
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        trace!("RobtopDeserializer::deserialize_option called on {:?}", self.peek_item());

        match self.peek_item() {
            Ok(None) | Err(Error::Eof) => {
                let _ = self.next_item(); // potentially skip the delimiter. Explicitly ignore the return value in case we have Error::Eof

                visitor.visit_none()
            },
            Err(err) => Err(err),
            Ok(Some(_)) => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_unit")
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_unit_struct")
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_newtype_struct")
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
        unimplemented!("deserialize_tuple")
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_tuple_struct")
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapAccess {
            deserializer: self,
            current_index: None,
            expected_fields: None,
        })
    }

    fn deserialize_struct<V>(
        self, _name: &'static str, fields: &'static [&'static str], visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        if self.map_like {
            visitor.visit_map(MapAccess {
                deserializer: self,
                current_index: None,
                expected_fields: Some(fields),
            })
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
        unimplemented!("deserialize_enum")
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        trace!("RobtopDeserializer::deserialize_identifier called on {:?}", self.peek_item());

        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Error<'de>>
    where
        V: Visitor<'de>,
    {
        unimplemented!("deserialize_ignored_any")
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
        trace!("Deserializing list entry '{:?}'", self.deserializer.peek_item());

        self.index += 1;

        match seed.deserialize(&mut *self.deserializer) {
            Err(Error::Eof) => Ok(None),
            Err(Error::Custom(message)) =>
                Err(Error::CustomAt {
                    message,
                    index: INDICES.get(self.index - 1).unwrap_or(&">=50"),
                }),
            Err(err) => Err(err),
            Ok(item) => Ok(Some(item)),
        }
    }
}

struct MapAccess<'a, 'de> {
    deserializer: &'a mut IndexedDeserializer<'de>,
    current_index: Option<&'de str>,
    expected_fields: Option<&'static [&'static str]>,
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error<'de>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error<'de>>
    where
        K: DeserializeSeed<'de>,
    {
        self.current_index = self.deserializer.peek_item().ok().flatten();

        if let (Some(expected), Some(index)) = (self.expected_fields, self.current_index) {
            if !expected.contains(&index) {
                warn!("Unexpected index {}", index);
            }
        }

        match seed.deserialize(&mut *self.deserializer) {
            Err(Error::Eof) => Ok(None),
            Err(err) => Err(err),
            Ok(item) => Ok(Some(item)),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error<'de>>
    where
        V: DeserializeSeed<'de>,
    {
        trace!(
            "Processing map entry '{:?}' '{:?}'",
            self.current_index,
            self.deserializer.peek_item()
        );

        match seed.deserialize(&mut *self.deserializer) {
            Err(Error::Custom(message)) =>
                Err(Error::CustomAt {
                    message,
                    index: self.current_index.unwrap(),
                }),
            r => r,
        }
    }
}
