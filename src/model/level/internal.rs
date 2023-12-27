use crate::{
    model::{
        level::{DemonRating, Level, LevelData, LevelLength, LevelRating},
        song::MainSong,
    },
    serde::InternalProxy,
    Dash,
};
use serde::{de::Error, Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Debug)]
struct InternalLevel<'src> {
    #[serde(rename = "1")]
    index_1: u64,
    #[serde(rename = "2")]
    index_2: &'src str,
    #[serde(rename = "3")]
    index_3: Option<&'src str>,
    #[serde(rename = "5")]
    index_5: u32,
    #[serde(rename = "6")]
    index_6: u64,
    #[serde(serialize_with = "crate::util::false_to_empty_string")]
    #[serde(rename = "25")]
    index_25: bool,
    #[serde(serialize_with = "crate::util::true_to_ten")]
    #[serde(rename = "8")]
    index_8: bool,
    #[serde(rename = "9")]
    index_9: i32,
    #[serde(serialize_with = "crate::util::false_to_empty_string")]
    #[serde(rename = "17")]
    index_17: bool,
    #[serde(rename = "10")]
    index_10: u32,
    #[serde(rename = "12")]
    index_12: u8,
    #[serde(rename = "13")]
    index_13: u8,
    #[serde(rename = "14")]
    index_14: i32,
    #[serde(rename = "15")]
    index_15: i32,
    #[serde(rename = "18")]
    index_18: u8,
    #[serde(rename = "19")]
    index_19: i32,
    #[serde(with = "crate::util::default_to_none")]
    #[serde(rename = "30")]
    index_30: Option<u64>,
    #[serde(rename = "31")]
    index_31: bool,
    #[serde(with = "crate::util::default_to_none")]
    #[serde(rename = "35")]
    index_35: Option<u64>,
    #[serde(rename = "37")]
    index_37: u8,
    #[serde(rename = "38")]
    index_38: bool,
    #[serde(with = "crate::util::default_to_none")]
    #[serde(rename = "39")]
    index_39: Option<u8>,
    #[serde(rename = "42")]
    index_42: bool,
    #[serde(rename = "43")]
    index_43: u8,
    #[serde(with = "crate::util::default_to_none")]
    #[serde(rename = "45")]
    index_45: Option<u32>,
    #[serde(rename = "46")]
    index_46: Option<&'src str>,
    #[serde(rename = "47")]
    index_47: Option<&'src str>,

    // Only present sometimes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4")]
    index_4: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "27")]
    index_27: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "28")]
    index_28: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "29")]
    index_29: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "36")]
    index_36: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "40")]
    index_40: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "52")]
    index_52: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "53")]
    index_53: Option<&'src str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "57")]
    index_57: Option<&'src str>,
}

impl<'de> Dash<'de> for Level<'de, (), Option<u64>, u64> {
    fn dash_deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = InternalLevel::deserialize(deserializer)?;

        Ok(Self {
            level_id: InternalProxy::from_deserialize_proxy(internal.index_1),
            name: InternalProxy::from_deserialize_proxy(internal.index_2),
            description: InternalProxy::from_deserialize_proxy(internal.index_3),
            version: InternalProxy::from_deserialize_proxy(internal.index_5),
            creator: InternalProxy::from_deserialize_proxy(internal.index_6),
            downloads: InternalProxy::from_deserialize_proxy(internal.index_10),
            gd_version: InternalProxy::from_deserialize_proxy(internal.index_13),
            likes: InternalProxy::from_deserialize_proxy(internal.index_14),
            length: InternalProxy::from_deserialize_proxy(internal.index_15),
            stars: InternalProxy::from_deserialize_proxy(internal.index_18),
            featured: InternalProxy::from_deserialize_proxy(internal.index_19),
            copy_of: InternalProxy::from_deserialize_proxy(internal.index_30),
            two_player: InternalProxy::from_deserialize_proxy(internal.index_31),
            custom_song: InternalProxy::from_deserialize_proxy(internal.index_35),
            coin_amount: InternalProxy::from_deserialize_proxy(internal.index_37),
            coins_verified: InternalProxy::from_deserialize_proxy(internal.index_38),
            stars_requested: InternalProxy::from_deserialize_proxy(internal.index_39),
            is_epic: InternalProxy::from_deserialize_proxy(internal.index_42),
            object_amount: InternalProxy::from_deserialize_proxy(internal.index_45),
            index_46: InternalProxy::from_deserialize_proxy(internal.index_46),
            index_47: InternalProxy::from_deserialize_proxy(internal.index_47),

            main_song: if internal.index_35.is_some() {
                None
            } else {
                Some(MainSong::from(internal.index_12))
            },
            difficulty: if !internal.index_8 {
                LevelRating::NotAvailable
            } else if internal.index_25 {
                LevelRating::Auto
            } else if internal.index_17 {
                LevelRating::Demon(DemonRating::from_response_value(internal.index_9))
            } else {
                LevelRating::from_response_value(internal.index_9)
            },
            level_data: (),
        })
    }

    fn dash_serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // We are in a manual impl, so we can do the .as_deref() trick to avoid needing two separate structs
        let index_3 = self.description.to_serialize_proxy();

        let internal = InternalLevel {
            index_1: self.level_id.to_serialize_proxy(),
            index_2: self.name.to_serialize_proxy(),
            index_3: index_3.as_deref(),
            index_5: self.version.to_serialize_proxy(),
            index_6: self.creator.to_serialize_proxy(),
            index_10: self.downloads.to_serialize_proxy(),
            index_13: self.gd_version.to_serialize_proxy(),
            index_14: self.likes.to_serialize_proxy(),
            index_15: self.length.to_serialize_proxy(),
            index_18: self.stars.to_serialize_proxy(),
            index_19: self.featured.to_serialize_proxy(),
            index_30: self.copy_of.to_serialize_proxy(),
            index_31: self.two_player.to_serialize_proxy(),
            index_35: self.custom_song.to_serialize_proxy(),
            index_37: self.coin_amount.to_serialize_proxy(),
            index_38: self.coins_verified.to_serialize_proxy(),
            index_39: self.stars_requested.to_serialize_proxy(),
            index_42: self.is_epic.to_serialize_proxy(),
            index_45: self.object_amount.to_serialize_proxy(),
            index_46: self.index_46.to_serialize_proxy(),
            index_47: self.index_47.to_serialize_proxy(),

            index_12: self.main_song.map(|song| song.main_song_id).unwrap_or(0),
            index_25: self.difficulty == LevelRating::Auto,
            index_8: self.difficulty != LevelRating::NotAvailable,
            index_9: self.difficulty.into_response_value(),
            index_17: self.difficulty.is_demon(),
            index_43: match self.difficulty {
                LevelRating::Demon(DemonRating::Easy) => 3,
                LevelRating::Demon(DemonRating::Medium) => 4,
                LevelRating::Demon(DemonRating::Hard) => 0,
                LevelRating::Demon(DemonRating::Insane) => 5,
                LevelRating::Demon(DemonRating::Extreme) => 6,
                _ => 5,
            },
            index_4: None,
            index_27: None,
            index_28: None,
            index_29: None,
            index_36: None,
            index_40: None,
            index_52: None,
            index_53: None,
            index_57: None,
        };
        internal.serialize(serializer)
    }
}

impl<'de> Dash<'de> for Level<'de, LevelData<'de>, Option<u64>, u64> {
    fn dash_deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = InternalLevel::deserialize(deserializer)?;

        let level_data = match (internal.index_4, internal.index_27, internal.index_28, internal.index_29) {
            (Some(idx4), Some(idx27), Some(idx28), Some(idx29)) => LevelData {
                level_data: InternalProxy::from_deserialize_proxy(idx4),
                password: InternalProxy::from_deserialize_proxy(idx27),
                time_since_upload: InternalProxy::from_deserialize_proxy(idx28),
                time_since_update: InternalProxy::from_deserialize_proxy(idx29),
                index_36: InternalProxy::from_deserialize_proxy(internal.index_36.unwrap_or_default()),
                index_40: InternalProxy::from_deserialize_proxy(internal.index_40.unwrap_or_default()),
                index_52: InternalProxy::from_deserialize_proxy(internal.index_52.unwrap_or_default()),
                index_53: InternalProxy::from_deserialize_proxy(internal.index_53.unwrap_or_default()),
                index_57: InternalProxy::from_deserialize_proxy(internal.index_57.unwrap_or_default()),
            },
            _ => return Err(D::Error::custom("Missing indices for level data!")),
        };

        Ok(Self {
            level_id: InternalProxy::from_deserialize_proxy(internal.index_1),
            name: InternalProxy::from_deserialize_proxy(internal.index_2),
            description: InternalProxy::from_deserialize_proxy(internal.index_3),
            version: InternalProxy::from_deserialize_proxy(internal.index_5),
            creator: InternalProxy::from_deserialize_proxy(internal.index_6),
            downloads: InternalProxy::from_deserialize_proxy(internal.index_10),
            gd_version: InternalProxy::from_deserialize_proxy(internal.index_13),
            likes: InternalProxy::from_deserialize_proxy(internal.index_14),
            length: InternalProxy::from_deserialize_proxy(internal.index_15),
            stars: InternalProxy::from_deserialize_proxy(internal.index_18),
            featured: InternalProxy::from_deserialize_proxy(internal.index_19),
            copy_of: InternalProxy::from_deserialize_proxy(internal.index_30),
            two_player: InternalProxy::from_deserialize_proxy(internal.index_31),
            custom_song: InternalProxy::from_deserialize_proxy(internal.index_35),
            coin_amount: InternalProxy::from_deserialize_proxy(internal.index_37),
            coins_verified: InternalProxy::from_deserialize_proxy(internal.index_38),
            stars_requested: InternalProxy::from_deserialize_proxy(internal.index_39),
            is_epic: InternalProxy::from_deserialize_proxy(internal.index_42),
            object_amount: InternalProxy::from_deserialize_proxy(internal.index_45),
            index_46: InternalProxy::from_deserialize_proxy(internal.index_46),
            index_47: InternalProxy::from_deserialize_proxy(internal.index_47),

            main_song: if internal.index_35.is_some() {
                None
            } else {
                Some(MainSong::from(internal.index_12))
            },
            difficulty: if !internal.index_8 {
                LevelRating::NotAvailable
            } else if internal.index_25 {
                LevelRating::Auto
            } else if internal.index_17 {
                LevelRating::Demon(DemonRating::from_response_value(internal.index_9))
            } else {
                LevelRating::from_response_value(internal.index_9)
            },

            level_data,
        })
    }

    fn dash_serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // We are in a manual impl, so we can do the .as_deref() trick to avoid needing two separate structs
        let index_3 = self.description.to_serialize_proxy();
        let index_4 = self.level_data.level_data.to_serialize_proxy();
        let index_27 = self.level_data.password.to_serialize_proxy();

        let internal = InternalLevel {
            index_1: self.level_id.to_serialize_proxy(),
            index_2: self.name.to_serialize_proxy(),
            index_3: index_3.as_deref(),
            index_5: self.version.to_serialize_proxy(),
            index_6: self.creator.to_serialize_proxy(),
            index_10: self.downloads.to_serialize_proxy(),
            index_13: self.gd_version.to_serialize_proxy(),
            index_14: self.likes.to_serialize_proxy(),
            index_15: self.length.to_serialize_proxy(),
            index_18: self.stars.to_serialize_proxy(),
            index_19: self.featured.to_serialize_proxy(),
            index_30: self.copy_of.to_serialize_proxy(),
            index_31: self.two_player.to_serialize_proxy(),
            index_35: self.custom_song.to_serialize_proxy(),
            index_37: self.coin_amount.to_serialize_proxy(),
            index_38: self.coins_verified.to_serialize_proxy(),
            index_39: self.stars_requested.to_serialize_proxy(),
            index_42: self.is_epic.to_serialize_proxy(),
            index_45: self.object_amount.to_serialize_proxy(),
            index_46: self.index_46.to_serialize_proxy(),
            index_47: self.index_47.to_serialize_proxy(),

            index_12: self.main_song.map(|song| song.main_song_id).unwrap_or(0),
            index_25: self.difficulty == LevelRating::Auto,
            index_8: self.difficulty != LevelRating::NotAvailable,
            index_9: self.difficulty.into_response_value(),
            index_17: self.difficulty.is_demon(),
            index_43: match self.difficulty {
                LevelRating::Demon(DemonRating::Easy) => 3,
                LevelRating::Demon(DemonRating::Medium) => 4,
                LevelRating::Demon(DemonRating::Hard) => 0,
                LevelRating::Demon(DemonRating::Insane) => 5,
                LevelRating::Demon(DemonRating::Extreme) => 6,
                _ => 5,
            },

            index_4: Some(index_4.borrow()),
            index_27: Some(index_27.borrow()),
            index_28: Some(self.level_data.time_since_upload.to_serialize_proxy()),
            index_29: Some(self.level_data.time_since_update.to_serialize_proxy()),
            index_36: Some(self.level_data.index_36.to_serialize_proxy()),
            index_40: Some(self.level_data.index_40.to_serialize_proxy()),
            index_52: Some(self.level_data.index_52.to_serialize_proxy()),
            index_53: Some(self.level_data.index_53.to_serialize_proxy()),
            index_57: Some(self.level_data.index_57.to_serialize_proxy()),
        };
        internal.serialize(serializer)
    }
}

impl InternalProxy for LevelLength {
    type DeserializeProxy = i32;
    type SerializeProxy<'a> = i32
    where
        Self: 'a;

    fn to_serialize_proxy(&self) -> Self::SerializeProxy<'_> {
        match *self {
            LevelLength::Unknown(unknown) => unknown,
            LevelLength::Tiny => 0,
            LevelLength::Short => 1,
            LevelLength::Medium => 2,
            LevelLength::Long => 3,
            LevelLength::ExtraLong => 4,
            LevelLength::Platformer => 5,
        }
    }

    fn from_deserialize_proxy(from: Self::DeserializeProxy) -> Self {
        match from {
            0 => LevelLength::Tiny,
            1 => LevelLength::Short,
            2 => LevelLength::Medium,
            3 => LevelLength::Long,
            4 => LevelLength::ExtraLong,
            5 => LevelLength::Platformer,
            int => LevelLength::Unknown(int),
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
