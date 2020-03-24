use crate::request::BaseRequest;
use serde::{Deserialize, Serialize};

/// Struct modelled after a request to `downloadGJLevel22.php`.
///
/// In the Geometry Dash API, this endpoint is used to download a level from
/// the servers and retrieve some additional information that isn't provided
/// with the response to a [`LevelsRequest`]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct LevelRequest<'a> {
    /// The base request data
    #[serde(borrow)]
    pub base: BaseRequest<'a>,

    /// The ID of the level to download
    ///
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// Some weird field the Geometry Dash Client sends along
    ///
    /// ## GD Internals:
    /// This value needs to be converted to an integer for the boomlings API
    pub inc: bool,

    /// Some weird field the Geometry Dash Client sends along
    ///
    /// ## GD Internals:
    /// This field is called `extras` in the boomlings API and needs to be
    /// converted to an integer
    pub extra: bool,
}

/// Enum representing the various filter states that can be achieved using the
/// `completed` and `uncompleted` options in the Geometry Dash client.
///
/// We can abuse this to either exclude a set of levels from a search or limit our search to a given
/// set of levels.
#[derive(Debug, Clone, Hash, Serialize, Deserialize, Default)]
pub struct CompletionFilter {
    /// The list of level ids to filter
    #[serde(rename = "completedLevels", default, skip_serializing_if = "Option::is_none")]
    // TODO: we have to get this wrapped inside parenthesis somehow
    ids: Option<Vec<u64>>,

    /// if `true`, only the levels matching the ids in [`ids`](CompletionFilter.ids) will
    /// be searched, if `false`, the levels in [`ids`](CompletionFilter.ids) will
    /// be excluded.
    ///
    /// Mutually exclusive with [`exclude_given`]
    #[serde(rename = "onlyCompleted")]
    only_search_given: bool,

    /// if `true`, the levels in [`ids`](CompletionFilter.ids) will be excluded, if `false`, only
    /// the levels matching the ids in [`ids`](CompletionFilter.ids) will be searched.
    ///
    /// Mutually exclusive with [`only_search_given`]
    #[serde(rename = "uncompleted")]
    exclude_given: bool,
}

impl CompletionFilter {
    /// Constructs a [`CompletionFilter`] that'll restrict the search to the
    /// list of provided ids
    pub const fn limit_search(ids: Vec<u64>) -> CompletionFilter {
        CompletionFilter {
            ids: Some(ids),
            only_search_given: true,
            exclude_given: false,
        }
    }

    /// Constructs a [`CompletionFilter`] that'll exclude the list of given ids
    /// from the search
    pub const fn exclude(ids: Vec<u64>) -> CompletionFilter {
        CompletionFilter {
            ids: Some(ids),
            only_search_given: false,
            exclude_given: true,
        }
    }
}

/// Struct containing the various search filters provided by the Geometry Dash
/// client.
#[derive(Debug, Default, Clone, Hash, Serialize, Deserialize)]
pub struct SearchFilters {
    /// In- or excluding levels that have already been beaten.
    ///
    /// Since outside the game the notion of "completing" a level is meaningless, this can be used
    /// to restrict the result a subset of an arbitrary set of levels, or exclude an arbitrary
    /// set of levels the result.
    ///
    /// ## GD Internals:
    /// This field abstracts away the `uncompleted`, `onlyCompleted` and
    /// `completedLevels` fields.
    ///
    /// * `uncompleted` is to be set to `1` if we wish to exclude completedlevels from the results
    ///   (and to `0` otherwise).
    /// * `onlyCompleted` is to be set to `1` if we wish to only search through completed levels
    ///   (and to `0` otherwise)
    /// * `completedLevels` is a list of levels ids that have been completed. It needs to be
    ///   provided if, and only if, either `uncompleted` or `onlyCompleted` are set to `1`. The ids
    ///   are comma seperated and enclosed by parenthesis.
    /// If no completion filtering is desired, both boolean fields are set to `0` and
    /// `completedLevels` is omitted.
    pub completion: CompletionFilter,

    /// Only retrieve featured levels
    ///
    /// ## GD Internals:
    /// This value needs to be converted to an integer for the boomlings API
    pub featured: bool,

    /// Only retrieve original (uncopied)  levels
    ///
    /// ## GD Internals:
    /// This value needs to be converted to an integer for the boomlings API
    pub original: bool,

    /// Only retrieve two-player levels
    ///
    /// ## GD Internals:
    /// This field is called `twoPlayer` in the boomlings API and needs to be
    /// converted to an integer
    #[serde(rename = "twoPlayer")]
    pub two_player: bool,

    /// Only retrieve levels with coins
    ///
    /// ## GD Internals:
    /// This value needs to be converted to an integer for the boomlings API
    pub coins: bool,

    /// Only retrieve epic levels
    ///
    /// ## GD Internals:
    /// This value needs to be converted to an integer for the boomlings API
    pub epic: bool,

    /// Only retrieve star rated levels
    ///
    /// ## GD Internals:
    /// This field is called `star` in the boomlings API and needs to be
    /// converted to an integer
    #[serde(rename = "star")]
    pub rated: bool,

    /// Optionally only retrieve levels that match the given `SongFilter`
    ///
    /// ## GD Internals:
    /// This field composes both the `customSong` and `song` fields of the
    /// boomlings API. To filter by main song, set the `song` field to the
    /// id of the main song, and omit the `customSong` field from the
    /// request. To filter
    /// by a newgrounds
    /// song, set `customSong`
    /// to `1` and `song` to the newgrounds ID of the custom song.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub song: Option<SongFilter>,
}

/// Enum containing the various types of
/// [`LevelsRequest`] possible
///
/// ## GD Internals:
/// + Unused values: `8`, `9`, `14`
/// + The values `15` and `17` are only used in Geometry Dash World and are the
/// same as `0` ([`LevelRequestType::Search`]) and `6` ([`LevelRequestType::Featured`]) respectively
#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum LevelRequestType {
    /// A search request.
    ///
    /// Setting this variant will enabled all the available search filters
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `0` in requests
    Search,

    /// Request to retrieve the list of most downloaded levels
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `1` in requests
    MostDownloaded,

    /// Request to retrieve the list of most liked levels
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `2` in requests
    MostLiked,

    /// Request to retrieve the list of treI which I understood more aboutnding levels
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `3` in requests
    Trending,

    /// Request to retrieve the list of most recent levels
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `4` in requests
    Recent,

    /// Retrieve levels by the user whose ID was specified in [`LevelsRequest::search_string`]
    /// (Note that is has to be the user Id, not the account id)
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `5` in requests
    User,

    /// Request to retrieve the list of featured levels, ordered by their
    /// [featured weight](::model::level::Featured::Featured) weight
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `6` in requests
    Featured,

    /// Request to retrieve a list of levels filtered by some magic criteria
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `7` in requests. According to the GDPS source,
    /// this simply looks for levels that have more than 9999 objects.
    Magic,

    /// Map pack levels. The search string is set to a comma seperated list of
    /// levels, which are the levels contained in the map pack
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `10` in requests
    MapPack,

    /// Request to retrieve the list of levels most recently awarded a rating.
    ///
    /// Using this option you can only receive levels that were awarded a rating in Geometry Dash
    /// 1.9 or later
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `11` in requests
    Awarded,

    /// Unknown how this works
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `12` in requests
    Followed,

    /// Unknown what this is
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `13` in requests
    Friends,

    /// Request to retrieve the levels in the hall of fame
    ///
    /// ## GD Internals:
    /// This variant is represented by the value `16` in requests.
    HallOfFame,

    /// Unknown variant not yet mapped by dash-rs
    Unknown(i32),
}

impl From<i32> for LevelRequestType {
    fn from(value: i32) -> Self {
        use LevelRequestType::*;

        match value {
            0 => Search,
            1 => MostDownloaded,
            2 => MostLiked,
            3 => Trending,
            4 => Recent,
            5 => User,
            6 => Featured,
            7 => Magic,
            10 => MapPack,
            11 => Awarded,
            12 => Followed,
            13 => Friends,
            16 => HallOfFame,
            _ => Unknown(value),
        }
    }
}

impl Into<i32> for LevelRequestType {
    fn into(self) -> i32 {
        match self {
            LevelRequestType::Search => 0,
            LevelRequestType::MostDownloaded => 1,
            LevelRequestType::MostLiked => 2,
            LevelRequestType::Trending => 3,
            LevelRequestType::Recent => 4,
            LevelRequestType::User => 5,
            LevelRequestType::Featured => 6,
            LevelRequestType::Magic => 7,
            LevelRequestType::MapPack => 10,
            LevelRequestType::Awarded => 11,
            LevelRequestType::Followed => 12,
            LevelRequestType::Friends => 13,
            LevelRequestType::HallOfFame => 16,
            LevelRequestType::Unknown(value) => value,
        }
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongFilter {
    #[serde(rename = "song")]
    song_id: u64,

    #[serde(rename = "customSong", skip_serializing_if = "is_false", default)]
    is_custom: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}
