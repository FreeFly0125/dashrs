use crate::{
    de::thunk::{ProcessError, Thunk},
    model::level::Password::PasswordCopy,
    util,
};
use base64::URL_SAFE;
use serde::{export::TryFrom, Deserialize, Serialize, Serializer};

/// Enum representing the possible level lengths known to dash-rs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
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

impl Into<i32> for LevelLength {
    fn into(self) -> i32 {
        match self {
            LevelLength::Unknown(unknown) => unknown,
            LevelLength::Tiny => 0,
            LevelLength::Short => 1,
            LevelLength::Medium => 2,
            LevelLength::Long => 3,
            LevelLength::ExtraLong => 4,
        }
    }
}

impl From<i32> for LevelLength {
    fn from(int: i32) -> Self {
        match int {
            0 => LevelLength::Tiny,
            1 => LevelLength::Short,
            2 => LevelLength::Medium,
            3 => LevelLength::Long,
            4 => LevelLength::ExtraLong,
            _ => LevelLength::Unknown(int),
        }
    }
}

/// Enum representing the possible level ratings
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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

/// Enum representing the possible demon difficulties
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(from = "&str")]
pub enum Password<'a> {
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

    /// The level requires the specified password to copy
    ///
    /// ## GD Internals
    /// The Geometry Dash servers communicate this variant by setting the password field in the
    /// following way:
    /// * Prepend a single `'1'` to the password
    /// * XOR the resulting string with the key `"26364"` (note that the XOR operation is performed
    ///   using the ASCII value of the characters in that string)
    /// * base64 encode the result of that
    #[serde(borrow)]
    PasswordCopy(Thunk<'a, DecodedPassword>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedPassword(String);

pub const LEVEL_PASSWORD_XOR_KEY: &str = "26364";

impl<'a> TryFrom<&'a str> for DecodedPassword {
    type Error = ProcessError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut decoded = base64::decode_config(value, URL_SAFE).map_err(ProcessError::Base64)?;

        util::cyclic_xor(&mut decoded, LEVEL_PASSWORD_XOR_KEY);

        // Geometry Dash adds an initial '0' character at the beginning that we don't care about, we just remove it
        decoded.remove(0);

        String::from_utf8(decoded)
            .map_err(|err| ProcessError::Utf8(err.utf8_error()))
            .map(DecodedPassword)
    }
}

impl Serialize for DecodedPassword {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        // Top level strats: We know exactly what size a password is going to have after base64 encoding
        // base64 theoretically do not perform any encoding at all, it only interprets data in sextets
        // instead of octets. Practically, these sextets are still displayed as bytes however. So 3 bytes of
        // data will lead to 4 bytes of encoded data. We know that a level password is 6 ASCII
        // digits long (6 bytes -> 8 encoded bytes) plus the "0" robtop adds for some reason (1 byte -> 2
        // bytes + 2 bytes padding). So we need a 12 byte buffer to encode the level password

        let mut password = [0u8, 7];

        password[0] = '0' as u8;
        // This is a strong assert because the copy_from_slice method would panic anyways.
        assert!(password[1..].len() == self.0.as_bytes().len(), "The level password size doesn't match.");
        password[1..].copy_from_slice(self.0.as_bytes());

        // We need to do the xor **before** we get the base64 encoded data
        util::cyclic_xor(&mut password, LEVEL_PASSWORD_XOR_KEY);

        // serialize_bytes does the base64 encode by itself
        serializer.serialize_bytes(&password)

    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(raw_password_data: &'a str) -> Self {
        match raw_password_data {
            "0" => Password::NoCopy,
            "Aw==" => Password::FreeCopy,
            _ => PasswordCopy(Thunk::Unprocessed(raw_password_data)),
        }
    }
}
