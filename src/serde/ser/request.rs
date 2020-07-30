//! Module containing a serializer for the data format robtop expects his requests to be in
//!
//! RobTop uses a non-standard variation of the `x-www-form-urlencoded` format for requests to
//! his servers. Since that format only have a very small intersection with actual
//! `x-www-form-urlencoded` it is easier to just implement a serializer from scratch instead of
//! using the `serde-urlencoded` crate. Experiences gained from GDCF have shown that using the
//! latter requires a _lot_ of `[serde(serialize_with = "...")]` attributes all over the code.
//!
//! The serializers makes the following assumptions, which makes it not standard-compliant:
//! * It does not replace spaces with '+' (RobTop's does not perform this conversion)
//! * It does not percent-encode unprintable/non-ASCII bytes (through the official client, inputting
//!   them isn't supported. What happens if we include them programmatically is something yet to be
//!   investigated) TODO GAME SPECIFIC

use crate::serde::SerError as Error;
use dtoa::Floating;
use itoa::Integer;
use serde::{
    ser::{Error as _, Impossible, SerializeStruct},
    Serialize, Serializer,
};
use std::{fmt::Display, io::Write};

#[allow(missing_debug_implementations)]
pub struct RequestSerializer<W> {
    writer: W,

    /// Value indicating whether this serializer has already serialized something. This is used to
    /// check if we need to prepend the delimiter to the next field.
    is_start: bool,
}

impl<W> RequestSerializer<W> {
    pub fn new(writer: W) -> Self {
        RequestSerializer { writer, is_start: true }
    }
}

macro_rules! unsupported {
    ($($method: ident: $t: ty),*) => {
        $(
            fn $method(self, _v: $t) -> Result<Self::Ok, Self::Error> {
                Err(Error::Unsupported(stringify!($method)))
            }
        )*
    };
}

impl<'a, W: Write> Serializer for &'a mut RequestSerializer<W> {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Impossible<(), Error>;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;

    unsupported!(
        serialize_bool: bool,
        serialize_i8: i8,
        serialize_i16: i16,
        serialize_i32: i32,
        serialize_i64: i64,
        serialize_u8: u8,
        serialize_u16: u16,
        serialize_u32: u32,
        serialize_u64: u64,
        serialize_f32: f32,
        serialize_f64: f64,
        serialize_char: char,
        serialize_str: &str,
        serialize_bytes: &[u8]
    );

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_none"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_some"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit_struct"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit_variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_newtype_struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_newtype_variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Unsupported("serialize_seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Unsupported("serialize_tuple"))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Unsupported("serialize_tuple_struct"))
    }

    fn serialize_tuple_variant(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Unsupported("serialize_tuple_variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Unsupported("serialize_map"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Unsupported("serialize_struct_variant"))
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        Err(Error::Unsupported("collect_str"))
    }
}

impl<'a, W: Write> SerializeStruct for &'a mut RequestSerializer<W> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.is_start {
            self.writer.write(b"&").map_err(Error::custom)?;
        }

        // we cannot do self.is_start = false here because the first field might be a struct that was
        // inlined, meaning no key/value pair is directly constructed! It has to happen when we actually
        // write the first key, which occurs inside some nested ValueSerializer call.

        value.serialize(&mut ValueSerializer {
            key: Some(key),
            serializer: self,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // All structs are inlined and flattened
        Ok(())
    }
}

struct ValueSerializer<'ser, W: Write> {
    key: Option<&'static str>,
    serializer: &'ser mut RequestSerializer<W>,
}

impl<'ser, W: Write> ValueSerializer<'ser, W> {
    fn write_key(&mut self) -> Result<(), Error> {
        if let Some(key) = self.key {
            self.serializer.writer.write_all(key.as_bytes()).map_err(Error::custom)?;
            self.serializer.writer.write(b"=").map_err(Error::custom)?;

            self.serializer.is_start = false;
        }
        Ok(())
    }

    fn write_integer<I: Integer>(&mut self, int: I) -> Result<(), Error> {
        self.write_key()?;

        itoa::write(&mut self.serializer.writer, int).map_err(Error::custom)?;

        Ok(())
    }

    fn write_float<F: Floating>(&mut self, float: F) -> Result<(), Error> {
        self.write_key()?;

        dtoa::write(&mut self.serializer.writer, float).map_err(Error::custom)?;

        Ok(())
    }
}

impl<'ser, 'a, W: Write> Serializer for &'a mut ValueSerializer<'ser, W> {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Impossible<(), Error>;
    type SerializeSeq = SerializeSeq<'a, W>;
    type SerializeStruct = &'a mut RequestSerializer<W>;
    type SerializeStructVariant = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_key()?;

        match v {
            true => self.serializer.writer.write(b"1"),
            false => self.serializer.writer.write(b"0"),
        }
        .map_err(Error::custom)?;

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_float(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_float(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.write_key()?;

        // We don't need allocations for appending a single char
        // A buffer of size 4 is always enough to encode a char
        let mut char_buffer: [u8; 4] = [0; 4];
        self.serializer
            .writer
            .write_all(v.encode_utf8(&mut char_buffer).as_bytes())
            .map_err(Error::custom)?;

        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_key()?;

        self.serializer.writer.write(v.as_bytes()).map_err(Error::custom)?;

        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.write_key()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit_struct"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit_variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_newtype_struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("serialize_newtype_variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.key.is_none() {
            return Err(Error::Unsupported("Nested sequences"));
        }

        self.write_key()?;

        // This is a horrible hack. In `LevelsRequest` there is one particular field, namely
        // 'completedLevels`, that represents a list of values. In the entire freaking API, this is the only
        // vector where serialization is required to surround the value list with parenthesis. We cannot
        // simply deal with this in a newtype wrapper around vec, since serde does not allows us (rightfully
        // so) to just randomly write parenthesis to an arbitrary serializer. Which is why we have to
        // special case that one field here, in the serializer for robtop's request data format.
        Ok(SerializeSeq {
            serializer: self.serializer,
            is_start: true,
            parenthesized: self.key == Some("completedLevels"),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Unsupported("serialize_tuple"))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Unsupported("serialize_tuple_struct"))
    }

    fn serialize_tuple_variant(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Unsupported("serialize_tuple_variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Unsupported("serialize_map"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        if self.key.is_none() {
            return Err(Error::Unsupported("struct inside sequence"));
        }

        // If we inline a struct, that struct might not be the first field we serialize. However, we do not
        // know whether the value we serialize is a struct or not until we reach this method, so the value
        // serializer will already have appended an '&' for us. This means that we do not want to add
        // another one for the first field of this nested struct.
        self.serializer.is_start = true;

        Ok(self.serializer)
    }

    fn serialize_struct_variant(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Unsupported("serialize_struct_variant"))
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        Err(Error::Unsupported("collect_str"))
    }
}

pub(crate) struct SerializeSeq<'ser, W: Write> {
    serializer: &'ser mut RequestSerializer<W>,
    is_start: bool,
    parenthesized: bool,
}

impl<'write, W: Write> serde::ser::SerializeSeq for SerializeSeq<'write, W> {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.is_start {
            self.serializer.writer.write(b",").map_err(Error::custom)?;
        } else if self.parenthesized {
            self.serializer.writer.write(b"(").map_err(Error::custom)?;
        }

        self.is_start = false;
        value.serialize(&mut ValueSerializer {
            key: None,
            serializer: self.serializer,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.is_start {
            self.serializer.writer.write(b"-").map_err(Error::custom)?; // empty sequence
        }
        if self.parenthesized {
            self.serializer.writer.write(b")").map_err(Error::custom)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{request::level::LevelRequest, serde::ser::request::RequestSerializer};
    use serde::Serialize;

    #[test]
    fn test_serialization() {
        let level_request = LevelRequest::default();
        let mut buffer = Vec::new();

        let mut ser = RequestSerializer {
            writer: &mut buffer,
            is_start: true,
        };
        let result = level_request.serialize(&mut ser);

        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&levelID=0&inc=0&extra=0",
            String::from_utf8(buffer).unwrap()
        );
    }
}
