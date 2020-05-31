//! Module containing all data models Geometry Dash uses
//!
//! For each data model there are two versions:
//! * A `Raw...` version which is the deserialization target. It can be constructed without any
//!   allocations at all and references the input it was deserialized from. Furthermore, these are a
//!   one-to-one mapping from response data into rust structures, meaning they also act as
//!   documentation of RobTop's data formats.
//! * A "Owned" version that owns all its fields
//!
//! The raw version can be converted into the owned version by cloning all the fields. The owned
//! version can produce a raw version by borrowing all fields (roughly speaking).

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod creator;
pub mod level;
pub mod song;

/// Enum modelling the version of a Geometry Dash client
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(into = "u8", from = "u8")]
pub enum GameVersion {
    /// Variant representing an unknown version. This variant is only used for
    /// levels that were uploaded before the game started tracking the
    /// version. This variant's string
    /// representation is `"10"`
    Unknown,

    /// Variant representing a the version represented by the given minor/major
    /// values in the form `major.minor`
    Version { minor: u8, major: u8 },
}
impl From<u8> for GameVersion {
    fn from(version: u8) -> Self {
        if version == 10 {
            GameVersion::Unknown
        } else {
            GameVersion::Version {
                major: (version / 10) as u8,
                minor: (version % 10) as u8,
            }
        }
    }
}

impl From<GameVersion> for u8 {
    fn from(version: GameVersion) -> Self {
        match version {
            GameVersion::Unknown => 10,
            GameVersion::Version { minor, major } => major * 10 + minor,
        }
    }
}

impl Display for GameVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GameVersion::Unknown => write!(f, "Unknown"),
            GameVersion::Version { minor, major } => write!(f, "{}.{}", major, minor),
        }
    }
}
