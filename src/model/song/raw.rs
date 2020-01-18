use crate::{
    de::thunk::{PercentDecoded, Thunk},
    model::song::NewgroundsSong,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::Utf8Error};

/// Struct modelling a [`NewgroundsSong`] the way it is represented by the Geometry Dash servers
///
/// See [`NewgroundSong`] for an owned version.
///
/// The Geometry Dash servers provide a list of the newgrounds songs of the
/// levels in a `getGJLevels` response.
///
/// ### Unused indices:
/// The following indices aren't used by the Geometry Dash servers: `9`
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawNewgroundsSong<'a> {
    /// This value is provided at index `1`
    #[serde(rename = "1")]
    pub song_id: u64,

    /// This value is provided at index `2`
    #[serde(rename = "2", borrow)]
    pub name: Cow<'a, str>,

    /// This value is provided at index `3`
    #[serde(rename = "3")]
    pub index_3: u64,

    /// This value is provided at index `4`
    #[serde(rename = "4")]
    pub artist: Cow<'a, str>,

    /// This value is provided at index `5`
    #[serde(rename = "5")]
    pub filesize: f64,

    /// This value is provided at index `6`
    #[serde(rename = "6")]
    pub index_6: Option<Cow<'a, str>>,

    /// This value is provided at index `7`
    #[serde(rename = "7")]
    pub index_7: Option<Cow<'a, str>>,

    /// This value is provided at index `8>`
    #[serde(rename = "8")]
    pub index_8: Cow<'a, str>,

    /// This value is provided at index `10`, and is percent encoded.
    #[serde(rename = "10")]
    pub link: Thunk<'a, PercentDecoded<'a>>,
}

impl<'a> RawNewgroundsSong<'a> {
    pub fn to_owned(self) -> Result<NewgroundsSong, Utf8Error> {
        Ok(NewgroundsSong {
            song_id: self.song_id,
            name: self.name.into_owned(),
            index_3: self.index_3,
            artist: self.artist.into_owned(),
            filesize: self.filesize,
            index_6: self.index_6.map(|cow| cow.into_owned()),
            index_7: self.index_7.map(|cow| cow.into_owned()),
            index_8: self.index_8.into_owned(),
            link: self.link.into_processed()?.0.into_owned(),
        })
    }
}

impl NewgroundsSong {
    pub fn as_raw(&self) -> RawNewgroundsSong {
        RawNewgroundsSong {
            song_id: self.song_id,
            name: Cow::Borrowed(self.name.as_ref()),
            index_3: self.index_3,
            artist: Cow::Borrowed(&self.artist),
            filesize: self.filesize,
            index_6: self.index_6.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
            index_7: self.index_7.as_ref().map(|s| Cow::Borrowed(s.as_ref())),
            index_8: Cow::Borrowed(&self.index_8),
            link: Thunk::Processed(PercentDecoded(Cow::Borrowed(&self.link))),
        }
    }
}
