use crate::{
    serde::{Internal, ProcessError},
    util,
};
use base64::URL_SAFE;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

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
#[cfg(test)]
mod tests {
    use crate::model::level::{robtop_encode_level_password, Password};
    use base64::URL_SAFE;

    #[test]
    fn deserialize_password() {
        assert_eq!(Password::from_robtop("AwcBBQAHAA=="), Ok(Password::PasswordCopy(123456)));
        assert_eq!(Password::from_robtop("AwUCBgU="), Ok(Password::PasswordCopy(3101)));
        assert_eq!(Password::from_robtop("AwYDBgQCBg=="), Ok(Password::PasswordCopy(0)));
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
