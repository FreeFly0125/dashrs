use crate::{
    model::{song::MainSong, GameVersion},
    serde::{Base64Decoded, Internal, ProcessError},
    util, Thunk,
};
use base64::URL_SAFE;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

mod internal;

/// Enum representing the possible level lengths known to dash-rs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LevelLength {
    /// Enum variant that's used by the [`From<i32>`](From) impl for when an
    /// unrecognized value is passed
    Unknown(i32),

    /// Tiny
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `0` in both requests and
    /// responses
    Tiny,

    /// Short
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in both requests and
    /// responses
    Short,

    /// Medium
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in both requests and
    /// responses
    Medium,

    /// Long
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in both requests and
    /// responses
    Long,

    /// Extra Long, sometime referred to as `XL`
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in both requests and
    /// responses
    ExtraLong,
}

/// Enum representing the possible level ratings
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LevelRating {
    /// Enum variant that's used by the [`From<i32>`](From) impl for when an
    /// unrecognized value is passed
    Unknown(i32),

    /// Not Available, sometimes referred to as `N/A` or `NA`
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `-1` in requests and by the
    /// value `0` in responses
    NotAvailable,

    /// Auto rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `-3` in requests, and not
    /// included in responses.
    Auto,

    /// Easy rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in requests and by the
    /// value `10` in responses
    Easy,

    /// Normal rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in requests and by the
    /// value `20` in responses
    Normal,

    /// Hard rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in requests and by the
    /// value `30` in responses
    Hard,

    /// Harder rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in requests and by the
    /// value `40` in responses
    Harder,

    /// Insane rating
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `5` in requests and by the
    /// value `50` in responses
    Insane,

    /// Demon rating.
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `-2` in requests. In
    /// responses, you will have to first check the provided level is a
    /// demon and then interpret the provided
    /// `rating` value as a [`DemonRating`]
    Demon(DemonRating),
}

impl LevelRating {
    pub fn is_demon(&self) -> bool {
        match self {
            LevelRating::Demon(_) => true,
            _ => false,
        }
    }
}

/// Enum representing the possible demon difficulties
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DemonRating {
    /// Enum variant that's used by the [`From<i32>`](From) impl for when an
    /// unrecognized value is passed
    Unknown(i32),

    /// Easy demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in requests and by the
    /// value `10` in responses
    Easy,

    /// Medium demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in requests and by the
    /// value `20` in responses
    Medium,

    /// Hard demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in requests and by the
    /// value `30` in responses
    Hard,

    /// Insane demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in requests and by the
    /// value `40` in responses
    Insane,

    /// Extreme demon
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `5` in requests and by the
    /// value `50` in responses
    Extreme,
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

    fn from_request_value(value: i32) -> DemonRating {
        match value {
            1 => DemonRating::Easy,
            2 => DemonRating::Medium,
            3 => DemonRating::Hard,
            4 => DemonRating::Insane,
            5 => DemonRating::Extreme,
            _ => DemonRating::Unknown(value),
        }
    }

    fn into_request_value(self) -> i32 {
        match self {
            DemonRating::Unknown(value) => value,
            DemonRating::Easy => 1,
            DemonRating::Medium => 2,
            DemonRating::Hard => 3,
            DemonRating::Insane => 4,
            DemonRating::Extreme => 5,
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

/// Enum representing a levels featured state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum Featured {
    /// The level isn't featured, and has never been featured before
    NotFeatured,

    /// The level isn't featured, but used to be (it either got unrated, or
    /// unfeatured, like Sonic Wave)
    Unfeatured,

    /// The level is featured, and has the contained value as its featured
    /// weight.
    ///
    /// The featured weight determines how high on the featured pages the level
    /// appear, where a higher value means a higher position.
    Featured(u32),
}

impl From<i32> for Featured {
    fn from(int: i32) -> Self {
        match int {
            -1 => Featured::Unfeatured,
            0 => Featured::NotFeatured,
            _ => Featured::Featured(int as u32),
        }
    }
}

impl Into<i32> for Featured {
    fn into(self) -> i32 {
        match self {
            Featured::NotFeatured => -1,
            Featured::Unfeatured => 0,
            Featured::Featured(weight) => weight as i32,
        }
    }
}

/// Enum representing a level's copyability status
// FIXME: Find a sane implementation for (de)serialize here
#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub enum Password {
    /// The level isn't copyable through the official Geometry Dash client
    ///
    /// ## GD Internals:
    /// The Geometry Dash servers communicate this variant by setting the password field to the
    /// literal string `"0"`, completely unencoded and unencrypted
    NoCopy,

    /// The level is free to copy
    ///
    /// ## GD Internals
    /// The Geometry Dash servers communicate this variant by setting the password field to the
    /// literal string `"Aw=="`. This is a base64 encoded version of the byte `0x3`, which in turn
    /// is the ASCII value of `'1'` XOR-ed with the ASCII value of `'2'`, the latter being the first
    /// character of the XOR key used for encoding passwords.
    FreeCopy,

    // We need to store only a u32, the Geometry Dash passwords are still way below this range
    // We just need to pad it with zeroes when serializing
    // Changing it to a u64 will be trivial
    /// The level requires the specified password to copy
    ///
    /// ## GD Internals
    /// The Geometry Dash servers communicate this variant by setting the password field in the
    /// following way:
    /// * Prepend a single `'1'` to the password
    /// * XOR the resulting string with the key `"26364"` (note that the XOR operation is performed
    ///   using the ASCII value of the characters in that string)
    /// * base64 encode the result of that
    PasswordCopy(u32),
}

pub const LEVEL_PASSWORD_XOR_KEY: &str = "26364";

// Move this out of the (De)Serialize impl so we can more easily test the functions
fn robtop_encode_level_password(pw: u32) -> [u8; 7] {
    let mut password = [b'0'; 7];
    password[0] = b'1';

    let mut itoa_buf = [0u8; 6];

    let n = itoa::write(&mut itoa_buf[..], pw).unwrap();

    // ensure the password is padded with zeroes as needed
    for i in 0..n {
        password[7 - n + i] = itoa_buf[i];
    }

    // We need to do the xor **before** we get the base64 encoded data
    util::cyclic_xor(&mut password[..], LEVEL_PASSWORD_XOR_KEY);

    password
}

impl Password {
    fn from_robtop(raw_password_data: &str) -> Result<Self, ProcessError> {
        Ok(match raw_password_data {
            "0" => Password::NoCopy,
            "Aw==" => Password::FreeCopy,
            _ => {
                // More than enough for storing the decoded password even if in future the format grows
                let mut decoded_buffer = [0; 32];
                let password_len =
                    base64::decode_config_slice(raw_password_data, URL_SAFE, &mut decoded_buffer).map_err(ProcessError::Base64)?;

                // This xor pass is applied after we base64 decoded the input, it's how the game tries to protect
                // data
                util::cyclic_xor(&mut decoded_buffer[..password_len], LEVEL_PASSWORD_XOR_KEY);

                // Geometry Dash adds an initial '1' character at the beginning that we don't care about, we just
                // skip it
                // The cost of UTF8 checking here is pretty much nothing since the password is so
                // small, no need to go unsafe
                // FIXME: no need to go through std::str APIs
                let decoded_str = std::str::from_utf8(&decoded_buffer[1..password_len]).expect("Password wasn't UTF-8 after a xor cycle.");
                let password = decoded_str.parse().map_err(ProcessError::IntParse)?;

                Password::PasswordCopy(password)
            },
        })
    }
}

impl Serialize for Internal<Password> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Password::FreeCopy => serializer.serialize_str("Aw=="),
            Password::NoCopy => serializer.serialize_str("0"),
            Password::PasswordCopy(pw) => {
                // serialize_bytes does the base64 encode by itself
                serializer.serialize_bytes(&robtop_encode_level_password(pw))
            },
        }
    }
}

impl<'de> Deserialize<'de> for Internal<Password> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_password_data = <&str>::deserialize(deserializer)?;

        Password::from_robtop(raw_password_data)
            .map(Internal)
            .map_err(serde::de::Error::custom)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Password::NoCopy => write!(f, "No Copy"),
            Password::FreeCopy => write!(f, "Free Copy"),
            Password::PasswordCopy(pw) => write!(f, "{:0>6}", pw),
        }
    }
}

/// Struct representing partial levels. These are returned to
/// [`LevelsRequest`]s and only
/// contain metadata
/// on the level.
///
/// ## GD Internals:
/// The Geometry Dash servers provide lists of partial levels via the
/// `getGJLevels` endpoint.
///
/// ### Unmapped values:
/// + Index `8`: Index 8 is a boolean value indicating whether the level has a
/// difficulty rating that isn't N/A. This is equivalent to checking if
/// [`PartialLevel::difficulty`] is unequal to
/// [`LevelRating::NotAvailable`]
/// + Index `17`: Index 17 is a boolean value indicating whether
/// the level is a demon level. This is equivalent to checking if
/// [`PartialLevel::difficulty`] is the [`LevelRating::Demon`] variant.
/// + Index `25`: Index 25 is a boolean value indicating
/// whether the level is an auto level. This is equivalent to checking if
/// [`PartialLevel::difficulty`] is equal to
/// [`LevelRating::Auto`].
///
/// ### Unprovided values:
/// These values are not provided for by the `getGJLevels` endpoint and are
/// thus only modelled in the [`Level`] struct: `4`, `27`,
/// `28`, `29`, `36`
///
/// ### Unused indices:
/// The following indices aren't used by the Geometry Dash servers: `11`, `16`,
/// `17`, `20`, `21`, `22`, `23`, `24`, `26`, `31`, `32`, `33`, `34`, `40`,
/// `41`, `44`
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Level<'a, Song, User> {
    /// The level's unique level id
    ///
    /// ## GD Internals:
    /// This value is provided at index `1`.
    pub level_id: u64,

    /// The level's name
    ///
    /// ## GD Internals:
    /// This value is provided at index `2`.
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    /// The level's description. Is [`None`] if the creator didn't put any
    /// description.
    ///
    /// ## GD Internals:
    /// This value is provided at index `3` and encoded using urlsafe base 64.
    pub description: Option<Thunk<'a, Base64Decoded<'a>>>,

    /// The [`PartialLevel`]'s version. The version get incremented every time
    /// the level is updated, and the initial version is always version 1.
    ///
    /// ## GD Internals:
    /// This value is provided at index `5`.
    pub version: u32,

    /// The ID of the level's creator
    ///
    /// ## GD Internals:
    /// This value is provided at index `6`.
    pub creator: User,

    /// The difficulty of this [`PartialLevel`]
    ///
    /// ## GD Internals:
    /// This value is a construct from the value at the indices `9`, `17` and
    /// `25`, whereas index 9 is an integer representation of either the
    /// [`LevelRating`] or the [`DemonRating`]
    /// struct, depending on the value of index 17.
    ///
    /// If index 25 is set to true, the level is an auto level and the value at
    /// index 9 is some nonsense, in which case it is ignored.
    pub difficulty: LevelRating,

    /// The amount of downloads
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`
    pub downloads: u32,

    /// The [`MainSong`] the level uses, if any.
    ///
    /// ## GD Internals:
    /// This value is provided at index `12`. Interpretation is additionally
    /// dependant on the value at index `35` (the custom song id), as
    /// without that information, a value of `0` for
    /// this field could either mean the level uses `Stereo Madness` or no
    /// main song.
    pub main_song: Option<MainSong>,

    /// The gd version the request was uploaded/last updated in.
    ///
    /// ## GD Internals:
    /// This value is provided at index `13`
    pub gd_version: GameVersion,

    /// The amount of likes this [`PartialLevel`] has received
    ///
    /// ## GD Internals:
    /// This value is provided at index `14`
    pub likes: i32,

    /// The length of this [`PartialLevel`]
    ///
    /// ## GD Internals:
    /// This value is provided as an integer representation of the
    /// [`LevelLength`] struct at index `15`
    pub length: LevelLength,

    /// The amount of stars completion of this [`PartialLevel`] awards
    ///
    /// ## GD Internals:
    /// This value is provided at index `18`
    pub stars: u8,

    /// This [`PartialLevel`]s featured state
    ///
    /// ## GD Internals:
    /// This value is provided at index `19`
    pub featured: Featured,

    /// The ID of the level this [`PartialLevel`] is a copy of, or [`None`], if
    /// this [`PartialLevel`] isn't a copy.
    ///
    /// ## GD Internals:
    /// This value is provided at index `30`
    pub copy_of: Option<u64>,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `31`
    pub index_31: Option<Cow<'a, str>>,

    /// The id of the newgrounds song this [`PartialLevel`] uses, or [`None`]
    /// if it useds a main song.
    ///
    /// ## GD Internals:
    /// This value is provided at index `35`, and a value of `0` means, that no
    /// custom song is used.
    pub custom_song: Song,

    /// The amount of coints in this [`PartialLevel`]
    ///
    /// ## GD Internals:
    /// This value is provided at index `37`
    pub coin_amount: u8,

    /// Value indicating whether the user coins (if present) in this
    /// [`PartialLevel`] are verified
    ///
    /// ## GD Internals:
    /// This value is provided at index `38`, as an integer
    pub coins_verified: bool,

    /// The amount of stars the level creator has requested when uploading this
    /// [`PartialLevel`], or [`None`] if no stars were requested.
    ///
    /// ## GD Internals:
    /// This value is provided at index `39`, and a value of `0` means no stars
    /// were requested
    pub stars_requested: Option<u8>,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `40`
    pub index_40: Option<Cow<'a, str>>,

    /// Value indicating whether this [`PartialLevel`] is epic
    ///
    /// ## GD Internals:
    /// This value is provided at index `42`, as an integer
    pub is_epic: bool,

    // TODO: figure this value out
    /// According to the GDPS source its a value called `starDemonDiff`. It
    /// seems to correlate to the level's difficulty.
    ///
    /// ## GD Internals:
    /// This value is provided at index `43` and seems to be an integer
    pub index_43: Cow<'a, str>,

    /// The amount of objects in this [`PartialLevel`]. Note that a value of `None` _does not_ mean
    /// that there are no objects in the level, but rather that the server's didn't provide an
    /// object count.
    ///
    /// ## GD Internals:
    /// This value is provided at index `45`, although only for levels uploaded
    /// in version 2.1 or later. For all older levels this is always `None`
    pub object_amount: Option<u32>,

    /// According to the GDPS source this is always `1`, although that is
    /// evidently wrong
    ///
    /// ## GD Internals:
    /// This value is provided at index `46` and seems to be an integer
    pub index_46: Option<Cow<'a, str>>,

    /// According to the GDPS source, this is always `2`, although that is
    /// evidently wrong
    ///
    /// ## GD Internals:
    /// This value is provided at index `47` and seems to be an integer
    pub index_47: Option<Cow<'a, str>>,

    /// Additional data about this level that can be retrieved by downloading the level.
    ///
    /// This is [`None`] for levels retrieved via the "overview" endpoint `getGJLevels`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_data: Option<LevelData<'a>>,
}

/// Struct encapsulating the additional level data returned when actually downloading a level
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelData<'a> {
    /// The level's actual data.
    ///
    /// TODO: Wrap in fitting Thunk
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`, and is urlsafe base64 encoded and `DEFLATE` compressed
    pub level_data: Cow<'a, str>,

    /// The level's password
    ///
    /// ## GD Internals:
    /// This value is provided at index `27`. For encoding details, see the documentation on the
    /// [`Password`] variants
    pub password: Password,

    /// The time passed since the `Level` was uploaded, as a string. Note that these strings are
    /// very imprecise, as they are only of the form "x months ago", or similar.
    ///
    /// TODO: Parse these into an enum
    ///
    /// ## GD Internals:
    /// This value is provided at index `28`
    pub time_since_upload: Cow<'a, str>,

    /// The time passed since the `Level` was last updated, as a string. Note that these strings are
    /// very imprecise, as they are only of the form "x months ago", or similar.
    ///
    /// ## GD Internals:
    /// This value is provided at index `29`
    pub time_since_update: Cow<'a, str>,

    /// According to the GDPS source, this is a value called `extraString`
    ///
    /// ## GD Internals:
    /// This value is provided at index `36`
    pub index_36: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use crate::model::level::{robtop_encode_level_password, Password};
    use base64::URL_SAFE;

    #[test]
    fn deserialize_password() {
        assert_eq!(Password::from_robtop("AwcBBQAHAA==").unwrap(), Password::PasswordCopy(123456));
        assert_eq!(Password::from_robtop("AwUCBgU=").unwrap(), Password::PasswordCopy(3101));
        assert_eq!(Password::from_robtop("AwYDBgQCBg==").unwrap(), Password::PasswordCopy(0));
        assert_eq!(Password::from_robtop("Aw==").unwrap(), Password::FreeCopy);
        assert_eq!(Password::from_robtop("0").unwrap(), Password::NoCopy);
    }

    #[test]
    fn serialize_password() {
        let encoded = robtop_encode_level_password(123456);
        let result = base64::encode_config(&encoded, URL_SAFE);

        assert_eq!(result, "AwcBBQAHAA==")
    }

    #[test]
    fn serialize_password_with_padding() {
        // TODO GAME SPECIFIC
        // in-game code for padding is inconsistent, see above test cases

        // password of 'Time Pressure' by AeonAir
        assert_eq!(base64::encode_config(&robtop_encode_level_password(3101), URL_SAFE), "AwYDBQUCBw==");
        // password of 'Breakthrough' by Hinds1324
        assert_eq!(base64::encode_config(&robtop_encode_level_password(0), URL_SAFE), "AwYDBgQCBg==")
    }
}
