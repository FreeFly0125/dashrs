use crate::ser::error::Error;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct IndexedSerializer {
    delimiter: &'static str,
    buffer: String,
    map_like: bool,
    is_start: bool,
}

impl IndexedSerializer {
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
    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.append(if v { "1" } else { "0" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.append(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serialize_bytes")
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
        unimplemented!("serialize_unit")
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serialize_unit_struct")
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serialize_unit_variant")
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_newtype_struct")
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self, name: &'static str, variant_index: u32, variant: &'static str, value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_newtype_variant")
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!("serialize_seq")
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!("serialize_tuple")
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!("serialize_tuple_struct")
    }

    fn serialize_tuple_variant(
        self, name: &'static str, variant_index: u32, variant: &'static str, len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!("serialize_tuple_variant")
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!("serialize_map")
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        // We don't store the struct name and the amount of fields doesn't matter
        Ok(self)
    }

    fn serialize_struct_variant(
        self, name: &'static str, variant_index: u32, variant: &'static str, len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!("serialize_struct_variant")
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        unimplemented!("collect_str")
    }
}

impl<'a> SerializeStructVariant for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_field")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
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

impl<'a> SerializeMap for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_key")
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_value")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_field")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_field")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_element")
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut IndexedSerializer {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!("serialize_element")
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
        };

        assert!(song.as_raw().serialize(&mut serializer).is_ok());
        println!("{}", serializer.buffer);
        assert_eq!(
            serializer.buffer,
            "1~|~771277~|~2~|~Creo - \
             Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~9.03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio%\
             2Engfiles%2Ecom%2F771000%2F771277%5FCreo%2D%2D%2DDune%2Emp3%3Ff1508708604"
        );
    }
}
