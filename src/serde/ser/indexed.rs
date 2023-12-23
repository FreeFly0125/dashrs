use crate::serde::ser::error::Error;
use dtoa::Float;
use itoa::{Buffer, Integer};
use serde::{
    ser::{Error as _, Impossible, SerializeStruct},
    Serialize, Serializer,
};
use std::{fmt::Display, io::Write};

#[allow(missing_debug_implementations)]
pub struct IndexedSerializer<W> {
    delimiter: &'static [u8],
    writer: W,
    map_like: bool,

    /// Value indicating whether this serializer has already serialized something. This is used to
    /// check if we need to prepend the delimiter to the next field.
    ///
    /// Note that this field cannot simply be replaced in favor of a `writer.len() == 0` check. In
    /// case of list-like serialization the first field could be `None`, which is serialized to the
    /// empty string. In that case, a delimiter needs to be appended, but since the writer would
    /// still be empty, no delimiter would be added.
    is_start: bool,
}

impl<W> IndexedSerializer<W>
where
    W: Write,
{
    pub fn new(delimiter: &'static str, writer: W, map_like: bool) -> Self {
        IndexedSerializer {
            delimiter: delimiter.as_bytes(),
            writer,
            map_like,
            is_start: true,
        }
    }

    fn append_integer<I: Integer>(&mut self, int: I) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.writer.write_all(self.delimiter)?;
        }

        let mut buffer = Buffer::new();
        self.writer.write(buffer.format(int).as_bytes()).map_err(Error::custom)?;

        Ok(())
    }

    fn append_float<F: Float>(&mut self, float: F) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.writer.write_all(self.delimiter)?;
        }

        let mut buffer = dtoa::Buffer::new();
        self.writer.write(buffer.format(float).as_bytes()).map_err(Error::custom)?;

        Ok(())
    }

    fn append(&mut self, s: &str) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.writer.write_all(self.delimiter)?;
        }

        self.writer.write_all(s.as_bytes())?;
        Ok(())
    }
}

impl<'a, W: Write> Serializer for &'a mut IndexedSerializer<W> {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Impossible<(), Error>;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.append(if v { "1" } else { "0" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.append_float(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.append_float(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // We don't need allocations for appending a single char
        // A buffer of size 4 is always enough to encode a char
        let mut char_buffer: [u8; 4] = [0; 4];
        self.append(v.encode_utf8(&mut char_buffer))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    // Here we serialize bytes by base64 encoding them, so it's always valid in Geometry Dash's format
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use base64::{engine::general_purpose::URL_SAFE, write::EncoderWriter};
        let mut enc = EncoderWriter::new(&mut self.writer, &URL_SAFE);
        enc.write_all(v)?;
        enc.finish()?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(self.delimiter)?;
        Ok(())
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
        // We don't store the struct name and the amount of fields doesn't matter
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

impl<'a, W: Write> SerializeStruct for &'a mut IndexedSerializer<W> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.map_like {
            self.append(key)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
