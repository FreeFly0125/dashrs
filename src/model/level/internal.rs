#![allow(unused_imports)]
use crate::{
    model::{
        level::{DemonRating, Featured, Level, LevelData, LevelLength, LevelRating, Objects, Password},
        song::MainSong,
        GameVersion,
    },
    serde::{Base64Decoded, IndexedDeserializer, IndexedSerializer},
    DeError, HasRobtopFormat, SerError,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    io::Write,
};

mod level_length {
    use crate::model::level::LevelLength;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(to_serialize: &LevelLength, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match to_serialize {
            LevelLength::Unknown(unknown) => serializer.serialize_i32(*unknown),
            LevelLength::Tiny => serializer.serialize_str("0"),
            LevelLength::Short => serializer.serialize_str("1"),
            LevelLength::Medium => serializer.serialize_str("2"),
            LevelLength::Long => serializer.serialize_str("3"),
            LevelLength::ExtraLong => serializer.serialize_str("4"),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<LevelLength, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match <&str>::deserialize(deserializer)? {
            "0" => LevelLength::Tiny,
            "1" => LevelLength::Short,
            "2" => LevelLength::Medium,
            "3" => LevelLength::Long,
            "4" => LevelLength::ExtraLong,
            int => LevelLength::Unknown(int.parse().map_err(D::Error::custom)?),
        })
    }
}

impl LevelRating {
    fn from_response_value(value: i32) -> LevelRating {
        match value {
            0 => LevelRating::NotAvailable,
            10 => LevelRating::Easy,
            20 => LevelRating::Normal,
            30 => LevelRating::Hard,
            40 => LevelRating::Harder,
            50 => LevelRating::Insane,
            _ => LevelRating::Unknown(value),
        }
    }

    fn into_response_value(self) -> i32 {
        match self {
            LevelRating::Unknown(value) => value,
            LevelRating::NotAvailable => 0,
            LevelRating::Easy => 10,
            LevelRating::Normal => 20,
            LevelRating::Hard => 30,
            LevelRating::Harder => 40,
            LevelRating::Insane => 50,
            LevelRating::Demon(demon_rating) => demon_rating.into_response_value(),
            _ => panic!("got {:?}, please handle before calling this function", self),
        }
    }
}

impl DemonRating {
    fn from_response_value(value: i32) -> DemonRating {
        match value {
            10 => DemonRating::Easy,
            20 => DemonRating::Medium,
            30 => DemonRating::Hard,
            40 => DemonRating::Insane,
            50 => DemonRating::Extreme,
            _ => DemonRating::Unknown(value),
        }
    }

    fn into_response_value(self) -> i32 {
        match self {
            DemonRating::Unknown(value) => value,
            DemonRating::Easy => 10,
            DemonRating::Medium => 20,
            DemonRating::Hard => 30,
            DemonRating::Insane => 40,
            DemonRating::Extreme => 50,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/partial_level.boilerplate"));
include!(concat!(env!("OUT_DIR"), "/level.boilerplate"));
