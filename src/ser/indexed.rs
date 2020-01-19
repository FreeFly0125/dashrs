use crate::ser::error::Error;
use itoa::Integer;
use serde::{
    ser::{Error as _, Impossible, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};
use std::fmt::Display;
use dtoa::Floating;

#[allow(missing_debug_implementations)]
pub struct IndexedSerializer {
    delimiter: &'static str,
    buffer: String,
    map_like: bool,

    /// Value indicating whether this serializer has already serialized something. This is used to
    /// check if we need to prepend the delimiter to the next field.
    ///
    /// Note that this field cannot simply be replaced in favor of a `buffer.len() == 0` check. In
    /// case of list-like serialization the first field could be `None`, which is serialized to the
    /// empty string. In that case, a delimiter needs to be appended, but since the buffer would
    /// still be empty, no delimiter would be added.
    is_start: bool,

    to_string_buf: [u8; 128], /* FIXME: what is longest possible string representation for a
                               * float/int? */
}

impl IndexedSerializer {
    fn append_integer<I: Integer>(&mut self, int: I) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.buffer += self.delimiter;
        }

        let len = itoa::write(&mut self.to_string_buf[..], int).map_err(Error::custom)?;

        self.buffer += unsafe { std::str::from_utf8_unchecked(&self.to_string_buf[..len]) };

        Ok(())
    }

    fn append_float<F: Floating>(&mut self, float: F) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.buffer += self.delimiter;
        }

        let len = dtoa::write(&mut self.to_string_buf[..], float).map_err(Error::custom)?;

        self.buffer += unsafe { std::str::from_utf8_unchecked(&self.to_string_buf[..len]) };

        Ok(())
    }

    fn append(&mut self, s: &str) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.buffer += self.delimiter;
        }

        self.buffer += s;

        Ok(())
    }
}

impl<'a> Serializer for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Impossible<(), Error>;
    type SerializeSeq = Self;
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
        self.append(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.buffer += self.delimiter;
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

impl<'a> SerializeStruct for &'a mut IndexedSerializer {
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

impl<'a> SerializeSeq for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(Error::Unsupported("SerializeSeq::serialize_element"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{model::song::NewgroundsSong, ser::indexed::IndexedSerializer};
    use serde::Serialize;

    #[test]
    fn serialize_creo_dune() {
        let song = NewgroundsSong {
            song_id: 771277,
            name: "Creo - Dune".to_string(),
            index_3: 50531,
            artist: "CreoMusic".to_owned(),
            filesize: 9.03,
            index_6: None,
            index_7: Some("UCsCWA3Y3JppL6feQiMRgm6Q".to_string()),
            index_8: "1".to_string(),
            link: "https://audio.ngfiles.com/771000/771277_Creo---Dune.mp3?f1508708604".to_string(),
        };

        let mut serializer = IndexedSerializer {
            delimiter: "~|~",
            buffer: "".to_string(),
            map_like: true,
            is_start: true,
            to_string_buf: [0; 128],
        };

        let ser_result = song.as_raw().serialize(&mut serializer);

        assert!(ser_result.is_ok(), "{:?}", ser_result);
        println!("{}", serializer.buffer);
        assert_eq!(
            serializer.buffer,
            "1~|~771277~|~2~|~Creo - \
             Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~9.03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio%\
             2Engfiles%2Ecom%2F771000%2F771277%5FCreo%2D%2D%2DDune%2Emp3%3Ff1508708604"
        );
    }
}
