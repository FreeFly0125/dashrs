//! Module containing all data models Geometry Dash uses
//!
//! For each data model there are two versions:
//! * A `Internal...` version which is the deserialization target. It can be constructed without any
//!   allocations at all and references the input it was deserialized from. Furthermore, these are
//!   nearly one-to-one mapping from response data into rust structures, meaning they also act as
//!   documentation of RobTop's data formats.
//! * A version that's public API and which abstracts over robtop's data format in a sensible way.
//!   These still borrow their data from the deserialization source, but can optionally own their
//!   contents. All data that would require allocations to be completely processed (such as base64
//!   encoded level descriptions) is put in [`Thunk`](crate::serde::thunk::Thunk)s and can be
//!   processed lazily on-demand. This allows us to deserialize the input and construct these
//!   representations with zero allocations.
//!
//! These versions can be converted to and from each other, simply by borrowing.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod comment;
pub mod creator;
pub mod level;
pub mod song;
pub mod user;

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
            GameVersion::Unknown => write!(f, "Pre 1.6"),
            GameVersion::Version { minor: 7, major: 0 } => write!(f, "1.6"),
            GameVersion::Version { minor, major } => write!(f, "{}.{}", major, minor),
        }
    }
}
