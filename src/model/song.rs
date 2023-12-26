use crate::serde::{GJFormat, PercentDecoder, ProcessError, Thunk};
use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};
use variant_partial_eq::VariantPartialEq;

/// Struct modelling a [`NewgroundsSong`]
///
/// ## GD Internals
/// The Geometry Dash servers provide a list of the newgrounds songs of the
/// levels in a `getGJLevels` response.
///
/// ### Unused indices:
/// The following indices aren't used by the Geometry Dash servers: `9`
#[derive(Debug, VariantPartialEq, Serialize, Deserialize, Clone, Dash)]
pub struct NewgroundsSong<'a> {
    /// The newgrounds id of this [`NewgroundsSong`]
    #[dash(index = 1)]
    pub song_id: u64,

    /// The name of this [`NewgroundsSong`]
    #[dash(index = 2)]
    pub name: Cow<'a, str>,

    #[dash(index = 3)]
    pub index_3: u64,

    /// The artist of this [`NewgroundsSong`]
    #[dash(index = 4)]
    pub artist: Cow<'a, str>,

    /// The filesize of this [`NewgroundsSong`], in megabytes
    #[dash(index = 5)]
    pub filesize: f64,

    #[dash(index = 6)]
    pub index_6: Option<Cow<'a, str>>,

    #[dash(index = 7)]
    pub index_7: Option<Cow<'a, str>>,

    #[dash(index = 8)]
    pub index_8: Cow<'a, str>,

    /// The direct `audio.ngfiles.com` download link for this [`NewgroundsSong`]
    #[serde(borrow)]
    #[dash(index = 10)]
    pub link: Thunk<'a, PercentDecoder>,
}

impl<'de> GJFormat<'de> for NewgroundsSong<'de> {
    const DELIMITER: &'static str = "~|~";
    const MAP_LIKE: bool = true;
}

impl<'a> NewgroundsSong<'a> {
    pub fn into_owned(self) -> Result<NewgroundsSong<'static>, ProcessError> {
        Ok(NewgroundsSong {
            song_id: self.song_id,
            name: Cow::Owned(self.name.into_owned()),
            index_3: self.index_3,
            artist: Cow::Owned(self.artist.into_owned()),
            filesize: self.filesize,
            index_6: self.index_6.map(|cow| Cow::Owned(cow.into_owned())),
            index_7: self.index_7.map(|cow| Cow::Owned(cow.into_owned())),
            index_8: Cow::Owned(self.index_8.into_owned()),
            link: Thunk::Processed(Cow::Owned(self.link.into_processed()?.into_owned())),
        })
    }
}

/// Struct representing Geometry Dash's main songs.
///
/// This data is not provided by the API and needs to be manually kept up to
/// date
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "u8")]
#[serde(into = "u8")]
pub struct MainSong {
    /// The ID of this [`MainSong`]
    pub main_song_id: u8,

    /// The name of this [`MainSong`]
    // even though we (de)serialize using From and Into, we have to mark these as skip so that the 'de lifetime isn't constrained by
    // 'static
    #[serde(skip)]
    pub name: &'static str,

    /// The artist of this [`MainSong`]
    #[serde(skip)]
    pub artist: &'static str,
}

impl MainSong {
    const fn new(main_song_id: u8, name: &'static str, artist: &'static str) -> MainSong {
        MainSong {
            main_song_id,
            name,
            artist,
        }
    }
}

/// All current [`MainSong`]s, as of Geometry Dash 2.1
pub const MAIN_SONGS: [MainSong; 22] = [
    MainSong::new(0, "Stereo Madness", "ForeverBound"),
    MainSong::new(1, "Back on Track", "DJVI"),
    MainSong::new(2, "Polargeist", "Step"),
    MainSong::new(3, "Dry Out", "DJVI"),
    MainSong::new(4, "Base after Base", "DJVI"),
    MainSong::new(5, "Can't Let Go", "DJVI"),
    MainSong::new(6, "Jumper", "Waterflame"),
    MainSong::new(7, "Time Machine", "Waterflame"),
    MainSong::new(8, "Cycles", "DJVI"),
    MainSong::new(9, "xStep", "DJVI"),
    MainSong::new(10, "Clutterfunk", "Waterflame"),
    MainSong::new(11, "Theory of Everything", "DJ-Nate"),
    MainSong::new(12, "Electroman ADventures", "Waterflame"),
    MainSong::new(13, "Clubstep", "DJ-Nate"),
    MainSong::new(14, "Electrodynamix", "DJ-Nate"),
    MainSong::new(15, "Hexagon Force", "Waterflame"),
    MainSong::new(16, "Blast Processing", "Waterflame"),
    MainSong::new(17, "Theory of Everything 2", "DJ-Nate"),
    MainSong::new(18, "Geometrical Dominator", "Waterflame"),
    MainSong::new(19, "Deadlocked", "F-777"),
    MainSong::new(20, "Fingerdash", "MDK"),
    MainSong::new(21, "Geomtry Dash", "MDK"),
];

/// Placeholder value for unknown [`MainSong`]s
///
/// When resolving a ['MainSong'] by its ID, but you pass a wrong ID, or
/// dash-rs hasn't updated to include the new model.song yet, you will receive this object
pub const UNKNOWN: MainSong = MainSong::new(
    0xFF,
    "The model.song was added after the release of dash-rs you're using",
    "Please either update to the newest version, or bug stadust about adding the new songs",
);

impl Display for NewgroundsSong<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "NewgroundsSong({}, {} by {})", self.song_id, self.name, self.artist)
    }
}

impl From<u8> for MainSong {
    fn from(song_id: u8) -> Self {
        *MAIN_SONGS.get(song_id as usize).unwrap_or(&UNKNOWN)
    }
}

impl From<MainSong> for u8 {
    fn from(song: MainSong) -> Self {
        song.main_song_id
    }
}
