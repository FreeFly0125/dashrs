use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod raw;

/// Struct representing a Newgrounds song.
///
/// Owned version of [`RawNewgroundsSong`]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewgroundsSong {
    /// The newgrounds id of this [`NewgroundsSong`]
    pub song_id: u64,

    /// The name of this [`NewgroundsSong`]
    pub name: String,

    pub index_3: u64,

    /// The artist of this [`NewgroundsSong`]
    pub artist: String,

    /// The filesize of this [`NewgroundsSong`], in megabytes
    pub filesize: f64,

    pub index_6: Option<String>,

    pub index_7: Option<String>,

    pub index_8: String,

    /// The direct `audio.ngfiles.com` download link for this [`NewgroundsSong`]
    pub link: String,
}

/// Struct representing Geometry Dash's main songs.
///
/// This data is not provided by the API and needs to be manually kept up to
/// date
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct MainSong {
    /// The ID of this [`MainSong`]
    pub main_song_id: u8,

    /// The name of this [`MainSong`]
    pub name: &'static str,

    /// The artist of this [`MainSong`]
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
pub const MAIN_SONGS: [MainSong; 21] = [
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
];

/// Placeholder value for unknown [`MainSong`]s
///
/// When resolving a main model.song by its ID, but you pass a wrong ID, or
/// GDCF hasn't updated to include the new model.song yet, you will receive this object
pub const UNKNOWN: MainSong = MainSong::new(
    0xFF,
    "The model.song was added after the release of GDCF you're using",
    "Please either update to the newest version, or bug stadust about adding the new songs",
);

impl Display for NewgroundsSong {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "NewgroundsSong({}, {} by {})", self.song_id, self.name, self.artist)
    }
}

impl From<u8> for &'static MainSong {
    fn from(song_id: u8) -> Self {
        MAIN_SONGS.get(song_id as usize).unwrap_or(&UNKNOWN)
    }
}
