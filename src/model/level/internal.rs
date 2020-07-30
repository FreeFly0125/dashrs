use crate::{
    model::{
        level::{DemonRating, Featured, Level, LevelData, LevelLength, LevelRating, Objects, Password},
        song::MainSong,
        GameVersion,
    },
    serde::{Base64Decoded, IndexedDeserializer, IndexedSerializer, Internal, RefThunk},
    DeError, HasRobtopFormat, SerError, Thunk,
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

mod ten_bool {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(to_serialize: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match to_serialize {
            true => serializer.serialize_str("10"),
            false => serializer.serialize_str("0"),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <&str>::deserialize(deserializer)? {
            "10" => Ok(true),
            "0" | "" => Ok(false),
            _ => Err(D::Error::custom("expected '10', '0' or the empty string")),
        }
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
            LevelRating::Easy => 20,
            LevelRating::Normal => 30,
            LevelRating::Hard => 40,
            LevelRating::Harder => 50,
            LevelRating::Insane => 60,
            LevelRating::Demon(demon_rating) => demon_rating.into_response_value(),
            _ => panic!("got {:?}, please handle before calling this function", self),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct InternalLevel<'a, 'b> {
    #[serde(rename = "1")]
    pub level_id: u64,

    #[serde(borrow, rename = "2")]
    pub name: &'a str,

    #[serde(rename = "3")]
    pub description: Option<Internal<Thunk<'a, Base64Decoded<'a>>>>,

    #[serde(rename = "5")]
    pub version: u32,

    #[serde(rename = "6")]
    pub creator: u64,

    #[serde(rename = "25")]
    pub is_auto: bool,

    #[serde(rename = "8", with = "ten_bool")]
    pub has_difficulty_rating: bool,

    #[serde(rename = "9")]
    pub rating: i32,

    #[serde(rename = "17")]
    pub is_demon: bool,

    #[serde(rename = "10")]
    pub downloads: u32,

    #[serde(rename = "12")]
    pub main_song: u8,

    #[serde(rename = "13")]
    pub gd_version: u8,

    #[serde(rename = "14")]
    pub likes: i32,

    #[serde(rename = "15", with = "level_length")]
    pub length: LevelLength,

    #[serde(rename = "18")]
    pub stars: u8,

    #[serde(rename = "19")]
    pub featured: Featured,

    #[serde(rename = "30", with = "crate::util::default_to_none")]
    pub copy_of: Option<u64>,

    #[serde(rename = "31")]
    pub index_31: Option<&'a str>,

    #[serde(rename = "35", with = "crate::util::default_to_none")]
    pub custom_song: Option<u64>,

    #[serde(rename = "37")]
    pub coin_amount: u8,

    #[serde(rename = "38")]
    pub coins_verified: bool,

    #[serde(rename = "39", with = "crate::util::default_to_none")]
    pub stars_requested: Option<u8>,

    #[serde(rename = "40")]
    pub index_40: Option<&'a str>,

    #[serde(rename = "42")]
    pub is_epic: bool,

    #[serde(rename = "43")]
    pub index_43: &'a str,

    #[serde(rename = "45", with = "crate::util::default_to_none")]
    pub object_amount: Option<u32>,

    #[serde(rename = "46")]
    pub index_46: Option<&'a str>,

    #[serde(rename = "47")]
    pub index_47: Option<&'a str>,

    #[serde(rename = "4", default, skip_serializing_if = "Option::is_none")]
    pub level_data: Option<RefThunk<'a, 'b, Objects>>,

    #[serde(rename = "27", default, skip_serializing_if = "Option::is_none")]
    pub password: Option<Internal<Password>>,

    #[serde(rename = "28", default, skip_serializing_if = "Option::is_none")]
    pub time_since_upload: Option<&'a str>,

    #[serde(rename = "29", default, skip_serializing_if = "Option::is_none")]
    pub time_since_update: Option<&'a str>,

    #[serde(rename = "36", default, skip_serializing_if = "Option::is_none")]
    pub index_36: Option<&'a str>,
}

impl<'a> HasRobtopFormat<'a> for Level<'a, Option<u64>, u64> {
    fn from_robtop_str(input: &'a str) -> Result<Self, DeError> {
        let internal = InternalLevel::deserialize(&mut IndexedDeserializer::new(input, ":", true))?;

        let level_data = match internal.level_data {
            None => None,
            Some(RefThunk::Unprocessed(level_string)) => Some(LevelData {
                level_data: Thunk::Unprocessed(level_string),
                password: internal.password.map(|pw| pw.0).unwrap_or(Password::NoCopy),
                time_since_upload: Cow::Borrowed(internal.time_since_upload.unwrap_or("Unknown")),
                time_since_update: Cow::Borrowed(internal.time_since_update.unwrap_or("Unknown")),
                index_36: internal.index_36.map(Cow::Borrowed),
            }),
            _ => unreachable!(),
        };

        Ok(Level {
            level_id: internal.level_id,
            name: Cow::Borrowed(internal.name),
            description: internal.description.map(|internal| internal.0),
            version: internal.version,
            creator: internal.creator,
            difficulty: if !internal.has_difficulty_rating {
                LevelRating::NotAvailable
            } else if internal.is_auto {
                LevelRating::Auto
            } else if internal.is_demon {
                LevelRating::Demon(DemonRating::from_response_value(internal.rating))
            } else {
                LevelRating::from_response_value(internal.rating)
            },
            downloads: internal.downloads,
            main_song: if internal.custom_song.is_some() {
                None
            } else {
                Some(MainSong::from(internal.main_song))
            },
            gd_version: GameVersion::from(internal.gd_version),
            likes: internal.likes,
            length: internal.length,
            stars: internal.stars,
            featured: internal.featured,
            copy_of: internal.copy_of,
            index_31: internal.index_31.map(Cow::Borrowed),
            custom_song: internal.custom_song,
            coin_amount: internal.coin_amount,
            coins_verified: internal.coins_verified,
            stars_requested: internal.stars_requested,
            index_40: internal.index_40.map(Cow::Borrowed),
            is_epic: internal.is_epic,
            index_43: Cow::Borrowed(internal.index_43),
            object_amount: internal.object_amount,
            index_46: internal.index_46.map(Cow::Borrowed),
            index_47: internal.index_47.map(Cow::Borrowed),
            level_data,
        })
    }

    fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
        let internal = InternalLevel {
            level_id: self.level_id,
            name: self.name.borrow(),
            description: self.description.as_ref().map(|thunk| {
                Internal(match thunk {
                    Thunk::Unprocessed(unproc) => Thunk::Unprocessed(unproc),
                    Thunk::Processed(Base64Decoded(moo)) => Thunk::Processed(Base64Decoded(Cow::Borrowed(moo.as_ref()))),
                })
            }),
            version: self.version,
            creator: self.creator,
            is_auto: self.difficulty == LevelRating::Auto,
            has_difficulty_rating: self.difficulty != LevelRating::NotAvailable,
            rating: self.difficulty.into_response_value(),
            is_demon: self.difficulty.is_demon(),
            downloads: self.downloads,
            main_song: self.main_song.map(|song| song.main_song_id).unwrap_or(0),
            gd_version: self.gd_version.into(),
            likes: self.likes,
            length: self.length,
            stars: self.stars,
            featured: self.featured,
            copy_of: self.copy_of,
            index_31: self.index_31.as_ref().map(|moo| moo.borrow()),
            custom_song: self.custom_song,
            coin_amount: self.coin_amount,
            coins_verified: self.coins_verified,
            stars_requested: self.stars_requested,
            index_40: self.index_40.as_ref().map(|moo| moo.borrow()),
            is_epic: self.is_epic,
            index_43: self.index_43.borrow(),
            object_amount: self.object_amount,
            index_46: self.index_46.as_ref().map(|moo| moo.borrow()),
            index_47: self.index_47.as_ref().map(|moo| moo.borrow()),
            level_data: self.level_data.as_ref().map(|data| data.level_data.as_ref_thunk()),
            password: self.level_data.as_ref().map(|data| Internal(data.password)),
            time_since_upload: self.level_data.as_ref().map(|data| data.time_since_upload.borrow()),
            time_since_update: self.level_data.as_ref().map(|data| data.time_since_update.borrow()),
            index_36: self
                .level_data
                .as_ref()
                .and_then(|data| data.index_36.as_ref())
                .map(|moo| moo.borrow()),
        };

        internal.serialize(&mut IndexedSerializer::new(":", writer, true))
    }
}
