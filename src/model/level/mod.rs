//! Module containing structs modelling Geometry Dash levels as they are returned from the boomlings
//! servers

use itoa::Buffer;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    io::Read,
};
use thiserror::Error;
use variant_partial_eq::VariantPartialEq;

use base64::{engine::general_purpose::URL_SAFE, Engine};
use flate2::read::{GzDecoder, GzEncoder, ZlibDecoder};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    model::{
        creator::Creator,
        level::{
            metadata::LevelMetadata,
            object::{speed::Speed, LevelObject, ObjectData},
        },
        song::{MainSong, NewgroundsSong},
        GameVersion,
    },
    serde::{Base64Decoder, ProcessError, Thunk, ThunkProcessor},
    util, Dash, GJFormat, SerError,
};
use flate2::Compression;

// use flate2::read::GzDecoder;
// use std::io::Read;

mod internal;
pub mod metadata;
pub mod object;

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

    /// Platformer levels (referred to as "Plat." on the level overview screens)
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `5` in both requests and
    /// responses
    Platformer,
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
    /// Returns true iff this [`LevelRating`] is the [`LevelRating::Demon`] variant
    pub fn is_demon(&self) -> bool {
        matches!(self, LevelRating::Demon(_))
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

/// Enum representing a levels featured state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum Featured {
    /// The level isn't featured, and has never been featured before
    ///
    /// ## GD Internals:
    /// In server responses, this variant is represented by the value `"0"`
    NotFeatured,

    /// The level isn't featured, but used to be (it either got unrated, or
    /// unfeatured, like Sonic Wave)
    ///
    /// ## GD Internals:
    /// In server responses, this variant is represented by the value `"-1"`
    Unfeatured,

    /// The level is featured, and has the contained value as its featured
    /// weight.
    ///
    /// The featured weight determines how high on the featured pages the level
    /// appear, where a higher value means a higher position.
    ///
    /// ## GD Internals:
    /// In server responses, this variant is represented simply by the contained weight
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

impl From<Featured> for i32 {
    fn from(featured: Featured) -> Self {
        match featured {
            Featured::NotFeatured => 0,
            Featured::Unfeatured => -1,
            Featured::Featured(weight) => weight as i32,
        }
    }
}

crate::into_conversion!(Featured, i32);

/// Enum representing a level's copyability status
// FIXME: Find a sane implementation for (de)serialize here
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
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
    /// In-Game, passwords are sometimes left-padded with zeros. However, this is not a requirement
    /// for the game to be able to correctly process passwords, and merely an implementation detail
    /// that changed at some point after 1.7
    PasswordCopy(u32),
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Password::NoCopy => serializer.serialize_none(),
            Password::FreeCopy => serializer.serialize_i32(-1),
            Password::PasswordCopy(password) => serializer.serialize_u32(*password),
        }
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let level_password = <Option<i32>>::deserialize(deserializer)?;

        match level_password {
            Some(-1) => Ok(Password::FreeCopy),
            Some(copy) => Ok(Password::PasswordCopy(copy as u32)),
            None => Ok(Password::NoCopy),
        }
    }
}

/// The XOR key the game uses to encode level passwords
pub const LEVEL_PASSWORD_XOR_KEY: &str = "26364";

/// Encodes the given numerical password by padding it with zeros and applies the XOR-encoding with
/// [`LEVEL_PASSWORD_XOR_KEY`]
fn robtop_encode_level_password(pw: u32) -> [u8; 7] {
    let mut password = [b'0'; 7];
    password[0] = b'1';

    let mut itoa_buf = Buffer::new();
    let formatted = itoa_buf.format(pw);

    let n = formatted.len();

    assert!(n <= 6);

    // ensure the password is padded with zeroes as needed
    for (i, b) in formatted.as_bytes().iter().enumerate() {
        password[7 - n + i] = *b;
    }

    // We need to do the xor **before** we get the base64 encoded data
    util::cyclic_xor(&mut password[..], LEVEL_PASSWORD_XOR_KEY);

    password
}

impl Password {
    /// Attempts to construct a [`Password`] instance from the raw-robtop provided data
    ///
    /// ## Arguments
    /// + `raw_password_data`: The raw data returned from the servers. Assumed to be follow the
    /// encoding described in [`Password`]'s documentation
    fn from_robtop(raw_password_data: &str) -> Result<Self, ProcessError> {
        Ok(match raw_password_data {
            "0" => Password::NoCopy,
            "Aw==" => Password::FreeCopy,
            _ => {
                // More than enough for storing the decoded password even if in future the format grows
                let mut decoded_buffer = [0; 32];
                let password_len = URL_SAFE.decode_slice(raw_password_data, &mut decoded_buffer)?;

                // This xor pass is applied after we base64 decoded the input, it's how the game tries to protect
                // data
                util::cyclic_xor(&mut decoded_buffer[..password_len], LEVEL_PASSWORD_XOR_KEY);

                // Geometry Dash adds an initial '1' character at the beginning that we don't care about, we just
                // skip it

                let mut password = 0;
                for byte in &decoded_buffer[1..password_len] {
                    password = password * 10 + (byte - b'0') as u32
                }
                Password::PasswordCopy(password)
            },
        })
    }
}

impl ThunkProcessor for Password {
    type Error = ProcessError;
    type Output<'a> = Password;

    fn from_unprocessed(unprocessed: Cow<str>) -> Result<Self, Self::Error> {
        Password::from_robtop(&unprocessed)
    }

    fn as_unprocessed<'b>(processed: &'b Self::Output<'_>) -> Result<Cow<'b, str>, Self::Error> {
        match *processed {
            Password::FreeCopy => Ok(Cow::Borrowed("Aw==")),
            Password::NoCopy => Ok(Cow::Borrowed("0")),
            Password::PasswordCopy(pw) => {
                // FIXME: its possible to avoid an allocation here by base64-encoding to a stack-buffer,
                // and passing that stack buffer directly to a Serializer's serialize_bytes method.
                Ok(Cow::Owned(URL_SAFE.encode(robtop_encode_level_password(pw))))
            },
        }
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

pub type ListedLevel<'a> = Level<'a, (), Option<NewgroundsSong<'a>>, Option<Creator<'a>>>;

/// Struct representing levels as returned by the boomlings API.
///
/// These can be retrieved using [`LevelRequest`](crate::request::level::LevelRequest)s or
/// [`LevelsRequest`](crate::request::level::LevelsRequest). The `level_data` field is only set if
/// the level was retrieved using a request of the former kind. For requests of the latter type, it
/// will be set to [`None`]
///
/// ## GD Internals:
/// The Geometry Dash servers provide lists of partial levels via the
/// `getGJLevels` endpoint. Complete levels can be downloaded via `downloadGJLevel`
///
/// ### Unmapped values:
/// + Index `8`: Index 8 is a boolean value indicating whether the level has a
/// difficulty rating that isn't N/A. This is equivalent to checking if
/// [`Level::difficulty`] is unequal to
/// [`LevelRating::NotAvailable`]
/// + Index `17`: Index 17 is a boolean value indicating whether
/// the level is a demon level. This is equivalent to checking if
/// [`Level::difficulty`] is the [`LevelRating::Demon`] variant.
/// + Index `25`: Index 25 is a boolean value indicating
/// whether the level is an auto level. This is equivalent to checking if
/// [`Level::difficulty`] is equal to
/// [`LevelRating::Auto`]
/// + Index `43`: This index is an indicator of demon difficulty as follows:
///  3 = easy demon,
///  4 = medium demon,
///  5 = insane demon,
///  6 = extreme demon.
/// In other cases it's hard demon (thanks Ryder!). However, since we extract this information from
/// index 9, dash-rs ignores this value.
///
/// ### Value only provided via `downloadGJLevels`
/// These values are not provided for by the `getGJLevels` endpoint and are
/// thus modelled in the [`LevelData`] struct: `4`, `27`,
/// `28`, `29`, `36`, `40`
///
/// ### Unused indices:
/// The following indices aren't used by the Geometry Dash servers: `11`, `16`,
/// `17`, `20`, `21`, `22`, `23`, `24`, `26`, `31`, `32`, `33`, `34`, `40`,
/// `41`, `44`
#[derive(Debug, VariantPartialEq, Serialize, Deserialize)]
pub struct Level<'a, Data = LevelData<'a>, Song = Option<u64>, User = u64> {
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
    #[variant_compare = "crate::util::option_variant_eq"]
    pub description: Option<Thunk<'a, Base64Decoder>>,

    /// The [`Level`]'s version. The version get incremented every time
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

    /// The difficulty of this [`Level`]
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

    /// The amount of likes this [`Level`] has received
    ///
    /// ## GD Internals:
    /// This value is provided at index `14`
    pub likes: i32,

    /// The length of this [`Level`]
    ///
    /// ## GD Internals:
    /// This value is provided as an integer representation of the
    /// [`LevelLength`] struct at index `15`
    pub length: LevelLength,

    /// The amount of stars completion of this [`Level`] awards. In the case of a platformer level, this 
    /// is instead the number of "moons" awarded.
    ///
    /// ## GD Internals:
    /// This value is provided at index `18`
    pub stars: u8,

    /// This [`Level`]s featured state
    ///
    /// ## GD Internals:
    /// This value is provided at index `19`
    pub featured: Featured,

    /// The ID of the level this [`Level`] is a copy of, or [`None`], if
    /// this [`Level`] isn't a copy.
    ///
    /// ## GD Internals:
    /// This value is provided at index `30`
    pub copy_of: Option<u64>,

    /// Value indicating whether this level is played in two-player mode
    ///
    /// ## GD Internals:
    /// This value is provided at index `31` and actually sanely encoded
    pub two_player: bool,

    /// The id of the newgrounds song this [`Level`] uses, or [`None`]
    /// if it useds a main song.
    ///
    /// ## GD Internals:
    /// This value is provided at index `35`, and a value of `0` means, that no
    /// custom song is used.
    pub custom_song: Song,

    /// The amount of coins in this [`Level`]
    ///
    /// ## GD Internals:
    /// This value is provided at index `37`
    pub coin_amount: u8,

    /// Value indicating whether the user coins (if present) in this
    /// [`Level`] are verified
    ///
    /// ## GD Internals:
    /// This value is provided at index `38`, as an integer
    pub coins_verified: bool,

    /// The amount of stars the level creator has requested when uploading this
    /// [`Level`], or [`None`] if no stars were requested.
    ///
    /// ## GD Internals:
    /// This value is provided at index `39`, and a value of `0` means no stars
    /// were requested
    pub stars_requested: Option<u8>,

    /// Value indicating whether this [`Level`] is epic
    ///
    /// ## GD Internals:
    /// This value is provided at index `42`, as an integer
    pub is_epic: bool,

    /// The amount of objects in this [`Level`]. Note that a value of `None` _does not_ mean
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
    pub level_data: Data,
}

impl<'a, Data, Song, User> Level<'a, Data, Song, User> {
    /// Returns `true` iff this level is a platformer level
    pub fn is_platformer(&self) -> bool {
        matches!(self.length, LevelLength::Platformer)
    }
}

impl<'de, Data, Song, User> GJFormat<'de> for Level<'de, Data, Song, User>
where
    Level<'de, Data, Song, User>: Dash<'de>,
{
    const DELIMITER: &'static str = ":";
    const MAP_LIKE: bool = true;
}

/// Struct encapsulating the additional level data returned when actually downloading a level
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LevelData<'a> {
    /// The level's actual data.
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`, and is urlsafe base64 encoded and `DEFLATE` compressed
    #[serde(borrow)]
    pub level_data: Thunk<'a, Objects>,

    /// The level's password
    ///
    /// ## GD Internals:
    /// This value is provided at index `27`. For encoding details, see the documentation on the
    /// [`Password`] variants
    pub password: Thunk<'a, Password>,

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
    pub index_36: Cow<'a, str>,

    pub index_40: Cow<'a, str>,

    pub index_52: Cow<'a, str>,

    pub index_53: Cow<'a, str>,

    pub index_57: Cow<'a, str>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Objects {
    pub meta: LevelMetadata,
    pub objects: Vec<LevelObject>,
}

#[derive(Debug, Error)]
pub enum LevelProcessError {
    #[error("{0}")]
    Deserialize(String),

    #[error("{0}")]
    Serialize(#[from] SerError),

    #[error("{0}")]
    Base64(#[from] base64::DecodeError),

    /// Unknown compression format for level data
    #[error("Unknown compression scheme")]
    UnknownCompression,

    /// Error during (de)compression
    #[error("{0}")]
    Compression(#[from] std::io::Error),

    /// The given level string did not contain a metadata section
    #[error("Missing metadata section in level string")]
    MissingMetadata,
}

impl ThunkProcessor for Objects {
    type Error = LevelProcessError;
    type Output<'a> = Objects;

    fn from_unprocessed(unprocessed: Cow<str>) -> Result<Self, LevelProcessError> {
        // Doing the entire base64 in one go is actually faster than using base64::read::DecoderReader and
        // having the two readers go back and forth.
        let decoded = URL_SAFE.decode(&*unprocessed)?;

        // Here's the deal: Robtop decompresses all levels by calling the zlib function 'inflateInit2_' with
        // the second argument set to 47. This basically tells zlib "this data might be compressed using
        // zlib or gzip format, with window size at most 15, but you gotta figure it out yourself".
        // However, flate2 doesnt expose this option, so we have to manually determine whether we
        // have gzip or zlib compression.

        let mut decompressed = String::new();

        match &decoded[..2] {
            // gz magic bytes
            [0x1f, 0x8b] => {
                let mut decoder = GzDecoder::new(&decoded[..]);

                decoder.read_to_string(&mut decompressed)?;
            },
            // There's no such thing as "zlib magic bytes", but the first byte stores some information about how the data is compressed.
            // '0x78' is the first byte for the compression method robtop used (note: this is only used for very old levels, as he switched
            // to gz for newer levels)
            [0x78, _] => {
                let mut decoder = ZlibDecoder::new(&decoded[..]);

                decoder.read_to_string(&mut decompressed)?;
            },
            _ => return Err(LevelProcessError::UnknownCompression),
        }

        let mut iter = decompressed.split_terminator(';');

        let metadata_string = match iter.next() {
            Some(meta) => meta,
            None => return Err(LevelProcessError::MissingMetadata),
        };

        let meta = LevelMetadata::from_gj_str(metadata_string).map_err(|err| LevelProcessError::Deserialize(err.to_string()))?;

        iter.map(LevelObject::from_gj_str)
            .collect::<Result<_, _>>()
            .map(|objects| Objects { meta, objects })
            .map_err(|err| LevelProcessError::Deserialize(err.to_string()))
    }

    fn as_unprocessed(processed: &Objects) -> Result<Cow<str>, LevelProcessError> {
        let mut bytes = Vec::new();

        processed.meta.write_gj(&mut bytes)?;

        bytes.push(b';');

        for object in &processed.objects {
            object.write_gj(&mut bytes)?;
            bytes.push(b';');
        }

        // FIXME(game specific): Should we remember the compression scheme (zlib or gz) from above, or just
        // always re-compress using gz? Since the game dyncamially detects the compression method, we're
        // compatible either way.

        let mut encoder = GzEncoder::new(&bytes[..], Compression::new(9)); // TODO: idk what these values mean
        let mut compressed = Vec::new();

        encoder.read_to_end(&mut compressed)?;

        Ok(Cow::Owned(URL_SAFE.encode(compressed)))
    }
}

impl Objects {
    pub fn length_in_seconds(&self) -> f32 {
        let mut portals = Vec::new();
        let mut furthest_x = 0.0;

        for object in &self.objects {
            if let ObjectData::SpeedPortal { checked: true, speed } = object.metadata {
                portals.push((object.x, speed))
            }

            furthest_x = f32::max(furthest_x, object.x);
        }

        portals.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());

        get_seconds_from_x_pos(furthest_x, self.meta.starting_speed, &portals)
    }
}

fn get_seconds_from_x_pos(pos: f32, start_speed: Speed, portals: &[(f32, Speed)]) -> f32 {
    let mut speed: f32 = start_speed.into();

    if portals.is_empty() {
        return pos / speed;
    }

    let mut last_obj_pos = 0.0;
    let mut total_time = 0.0;

    for (x, portal_speed) in portals {
        // distance between last portal and this one
        let current_segment = x - last_obj_pos;

        // break if we're past the position we want to calculate the position to
        if pos <= current_segment {
            break;
        }

        // Calculate time spent in this segment and add to total time
        total_time += current_segment / speed;

        speed = (*portal_speed).into();

        last_obj_pos = *x;
    }

    // add the time spent between end and last portal to total time and return
    (pos - last_obj_pos) / speed + total_time
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::URL_SAFE, Engine};

    use crate::model::level::{robtop_encode_level_password, Password};

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
        let result = URL_SAFE.encode(&encoded);

        assert_eq!(result, "AwcBBQAHAA==")
    }

    #[test]
    fn serialize_password_with_padding() {
        // TODO GAME SPECIFIC
        // in-game code for padding is inconsistent, see above test cases

        // password of 'Time Pressure' by AeonAir
        assert_eq!(URL_SAFE.encode(&robtop_encode_level_password(3101)), "AwYDBQUCBw==");
        // password of 'Breakthrough' by Hinds1324
        assert_eq!(URL_SAFE.encode(&robtop_encode_level_password(0)), "AwYDBgQCBg==")
    }
}
