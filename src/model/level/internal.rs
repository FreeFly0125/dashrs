use crate::{
    model::{
        level::{DemonRating, Featured, LevelLength, LevelRating, PartialLevel},
        song::{MainSong, MAIN_SONGS},
        GameVersion,
    },
    serde::{Base64Decoded, HasRobtopFormat, Internal},
    Thunk,
};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

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

    fn from_request_value(value: i32) -> LevelRating {
        match value {
            -3 => LevelRating::Auto,
            -2 => LevelRating::Demon(DemonRating::Unknown(-1)), /* The value doesn't matter, since setting the request field "rating" to
                                                                  * -2 means "search for any demon, regardless of difficulty" */
            -1 => LevelRating::NotAvailable,
            1 => LevelRating::Easy,
            2 => LevelRating::Normal,
            3 => LevelRating::Hard,
            4 => LevelRating::Harder,
            5 => LevelRating::Insane,
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

    fn into_request_value(self) -> i32 {
        match self {
            LevelRating::Unknown(value) => value,
            LevelRating::NotAvailable => -1,
            LevelRating::Auto => -3,
            LevelRating::Easy => 1,
            LevelRating::Normal => 2,
            LevelRating::Hard => 3,
            LevelRating::Harder => 4,
            LevelRating::Insane => 5,
            LevelRating::Demon(_) => -2,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalPartialLevel<'a> {
    #[serde(rename = "1")]
    pub level_id: u64,

    #[serde(borrow, rename = "2")]
    pub name: Cow<'a, str>,

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
    pub main_song: MainSong,

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
    pub index_31: Option<Cow<'a, str>>,

    #[serde(rename = "35", with = "crate::util::default_to_none")]
    pub custom_song: Option<u64>,

    #[serde(rename = "37")]
    pub coin_amount: u8,

    #[serde(rename = "38")]
    pub coins_verified: bool,

    #[serde(rename = "39", with = "crate::util::default_to_none")]
    pub stars_requested: Option<u8>,

    #[serde(rename = "40")]
    pub index_40: Option<Cow<'a, str>>,

    #[serde(rename = "42")]
    pub is_epic: bool,

    #[serde(rename = "43")]
    pub index_43: Cow<'a, str>,

    #[serde(rename = "45", with = "crate::util::default_to_none")]
    pub object_amount: Option<u32>,

    #[serde(rename = "46")]
    pub index_46: Option<Cow<'a, str>>,

    #[serde(rename = "47")]
    pub index_47: Option<Cow<'a, str>>,
}

impl<'a> HasRobtopFormat<'a> for PartialLevel<'a, Option<u64>, u64> {
    type Internal = InternalPartialLevel<'a>;

    const DELIMITER: &'static str = ":";
    const MAP_LIKE: bool = true;

    fn as_internal(&'a self) -> Self::Internal {
        InternalPartialLevel {
            level_id: self.level_id,
            name: Cow::Borrowed(self.name.borrow()),
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
            main_song: self.main_song.unwrap_or(MAIN_SONGS[0]),
            gd_version: self.gd_version.into(),
            likes: self.likes,
            length: self.length,
            stars: self.stars,
            featured: self.featured,
            copy_of: self.copy_of,
            index_31: self.index_31.as_ref().map(|moo| Cow::Borrowed(moo.borrow())),
            custom_song: self.custom_song,
            coin_amount: self.coin_amount,
            coins_verified: self.coins_verified,
            stars_requested: self.stars_requested,
            index_40: self.index_40.as_ref().map(|moo| Cow::Borrowed(moo.borrow())),
            is_epic: self.is_epic,
            index_43: Cow::Borrowed(self.index_43.borrow()),
            object_amount: self.object_amount,
            index_46: self.index_46.as_ref().map(|moo| Cow::Borrowed(moo.borrow())),
            index_47: self.index_47.as_ref().map(|moo| Cow::Borrowed(moo.borrow())),
        }
    }

    fn from_internal(int: Self::Internal) -> Self {
        PartialLevel {
            level_id: int.level_id,
            name: int.name,
            description: int.description.map(|internal| internal.0),
            version: int.version,
            creator: int.creator,
            difficulty: if !int.has_difficulty_rating {
                LevelRating::NotAvailable
            } else if int.is_auto {
                LevelRating::Auto
            } else if int.is_demon {
                LevelRating::Demon(DemonRating::from_response_value(int.rating))
            } else {
                LevelRating::from_response_value(int.rating)
            },
            downloads: int.downloads,
            main_song: if int.custom_song.is_some() { None } else { Some(int.main_song) },
            gd_version: GameVersion::from(int.gd_version),
            likes: int.likes,
            length: int.length,
            stars: int.stars,
            featured: int.featured,
            copy_of: int.copy_of,
            index_31: int.index_31,
            custom_song: int.custom_song,
            coin_amount: int.coin_amount,
            coins_verified: int.coins_verified,
            stars_requested: int.stars_requested,
            index_40: int.index_40,
            is_epic: int.is_epic,
            index_43: int.index_43,
            object_amount: int.object_amount,
            index_46: int.index_46,
            index_47: int.index_47,
        }
    }
}
