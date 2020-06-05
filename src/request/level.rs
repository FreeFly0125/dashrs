use crate::{
    model::{
        level::{DemonRating, LevelLength, LevelRating},
        song::MainSong,
    },
    request::BaseRequest,
};
use serde::{Deserialize, Serialize, Serializer};

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
    /// Mutually exclusive with [`CompletionFilter::exclude_given`]
    #[serde(rename = "onlyCompleted")]
    only_search_given: bool,

    /// if `true`, the levels in [`ids`](CompletionFilter.ids) will be excluded, if `false`, only
    /// the levels matching the ids in [`ids`](CompletionFilter.ids) will be searched.
    ///
    /// Mutually exclusive with [`CompletionFilter::only_search_given`]
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
}

impl SearchFilters {
    /// Limit search results to star rated levels
    pub const fn rated(mut self) -> Self {
        self.rated = true;
        self
    }

    /// Limit search results to epic levels
    pub const fn epic(mut self) -> Self {
        self.epic = true;
        self
    }

    /// Limit search results to levels with coins
    pub const fn has_coins(mut self) -> Self {
        self.coins = true;
        self
    }

    /// Limit search results to levels in two player mode (that is, where controls in dual mode are
    /// split)
    pub const fn two_player(mut self) -> Self {
        self.two_player = true;
        self
    }

    /// Limit search results to levels that are not copies of other levels
    pub const fn original(mut self) -> Self {
        self.original = true;
        self
    }

    /// Limit search results to featured levels
    pub const fn featured(mut self) -> Self {
        self.featured = true;
        self
    }

    pub fn completion_filter(mut self, filter: CompletionFilter) -> Self {
        self.completion = filter;
        self
    }

    /// Limit search results to levels with the given [`MainSong`]
    pub fn main_song(mut self, main_song: MainSong) -> Self {
        self.song = Some(SongFilter {
            song_id: main_song.main_song_id as u64,
            is_custom: false,
        });
        self
    }

    /// Limit search results to levels that use a custom song matching the given id.
    pub fn custom_song(mut self, song_id: u64) -> Self {
        self.song = Some(SongFilter { song_id, is_custom: true });
        self
    }
}

/// Enum containing the various types of [`LevelsRequest`] possible
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
    /// [featured weight](crate::model::level::Featured) weight
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

impl Default for LevelRequestType {
    fn default() -> Self {
        LevelRequestType::Search
    }
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

/// Struct modelled after a request to `getGJLevels21.php`
///
/// In the Geometry Dash API, this endpoint is used to retrieve a list of
/// levels matching the specified criteria, along with their
/// [`NewgroundsSong`](crate::model::song::NewgroundsSong)s and
/// [`Creator`](crate::model::creator::Creator)s
#[derive(Debug, Default, Clone, Serialize)]
pub struct LevelsRequest<'a> {
    /// The base request data
    #[serde(borrow)]
    pub base: BaseRequest<'a>,

    /// The type of level list to retrieve
    ///
    /// ## GD Internals:
    /// This field is called `type` in the boomlings API and needs to be
    /// converted to an integer
    #[serde(rename = "type")]
    pub request_type: LevelRequestType,

    /// A search string to filter the levels by
    ///
    /// This value is ignored unless [`LevelsRequest::request_type`] is set to
    /// [`LevelRequestType::Search`] or [`LevelRequestType::User`]
    ///
    /// ## GD Internals:
    /// This field is called `str` in the boomlings API
    #[serde(rename = "str")]
    pub search_string: &'a str,

    /// A list of level lengths to filter by
    ///
    /// This value is ignored unless [`LevelsRequest::request_type`] is set to
    /// [`LevelRequestType::Search`]
    ///
    /// ## GD Internals:
    /// This field is called `len` in the boomlings API and needs to be
    /// converted to a comma separated list of integers, or a single dash
    /// (`-`) if filtering by level length isn't wanted.
    #[serde(rename = "len")]
    lengths: Vec<LengthFilter>,

    /// A list of level ratings to filter by.
    ///
    /// To filter by any demon, add [`LevelRating::Demon`] with any arbitrary [`DemonRating`] value.
    ///
    /// `ratings` and [`LevelsRequest::demon_rating`] are mutually exlusive.
    ///
    /// This value is ignored unless [`LevelsRequest::request_type`] is set to
    /// [`LevelRequestType::Search`]
    ///
    /// ## GD Internals:
    /// This field is called `diff` in the boomlings API and needs to be
    /// converted to a comma separated list of integers, or a single dash
    /// (`-`) if filtering by level rating isn't wanted.
    #[serde(rename = "diff")]
    ratings: Vec<RatingFilter>,

    /// Optionally, a single demon rating to filter by. To filter by any demon
    /// rating, use [`LevelsRequest::ratings`]
    ///
    /// `demon_rating` and `ratings` are mutually exlusive.
    ///
    /// This value is ignored unless [`LevelsRequest::request_type`] is set to
    /// [`LevelRequestType::Search`]
    ///
    /// ## GD Internals:
    /// This field is called `demonFilter` in the boomlings API and needs to be
    /// converted to an integer. If filtering by demon rating isn't wanted,
    /// the value has to be omitted from the request.
    #[serde(rename = "demonFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    demon_rating: Option<DemonFilter>,

    /// The page of results to retrieve
    pub page: u32,

    /// Some weird value the Geometry Dash client sends along
    pub total: i32,

    /// Search filters to apply.
    ///
    /// This value is ignored unless [`LevelsRequest::request_type`] is set to
    /// [`LevelRequestType::Search`]
    pub search_filters: SearchFilters,
}

impl<'a> LevelsRequest<'a> {
    const_setter!(page: u32);

    const_setter!(total: i32);

    const_setter!(request_type: LevelRequestType);

    pub fn with_base(base: BaseRequest<'a>) -> Self {
        LevelsRequest {
            base,
            ..Default::default()
        }
    }

    /// Turns this request into a [`LevelRequestType::Search`]-type request, with the search
    /// parameter set to the given string
    pub const fn search(mut self, search_string: &'a str) -> Self {
        self.search_string = search_string;
        self.request_type = LevelRequestType::Search;
        self
    }

    /// Turns on filtering by level length (if not already on) and adds the given level length to
    /// the list of lengths to include in the search results
    pub fn with_length(mut self, length: LevelLength) -> Self {
        self.lengths.push(LengthFilter(length));
        self
    }

    /// Turns on filtering by level rating (if not already on) and adds the given level rating to
    /// the list of ratings to include in the search results
    ///
    /// Passing [`LevelRating::Demon`] here will turn on filtering by _any_ demon difficulty. The
    /// filter `demon_rating` for specific demon difficulties is reset to `None` when this method is
    /// called, as these modes are mutually exclusive.
    pub fn with_rating(mut self, rating: LevelRating) -> Self {
        self.demon_rating = None;
        self.ratings.push(RatingFilter(rating));
        self
    }

    /// Turns on filtering by demon difficulty
    ///
    /// Resets any [`LevelRating`] filters set beforehand, as these modes are mutually exclusive.
    pub fn demon_rating(mut self, demon_rating: DemonRating) -> Self {
        self.ratings.clear();
        self.demon_rating = Some(DemonFilter(demon_rating));
        self
    }

    pub fn search_filters(mut self, filters: SearchFilters) -> Self {
        self.search_filters = filters;
        self
    }
}

/// Newtype struct for [`DemonRating`] to implement robtop's serialization for requests on
#[derive(Debug, Clone, Copy)]
struct DemonFilter(DemonRating);

impl Serialize for DemonFilter {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let numerical_value = match self.0 {
            DemonRating::Unknown(value) => value,
            DemonRating::Easy => 1,
            DemonRating::Medium => 2,
            DemonRating::Hard => 3,
            DemonRating::Insane => 4,
            DemonRating::Extreme => 5,
        };

        serializer.serialize_i32(numerical_value)
    }
}

/// Newtype struct for [`LevelLength`] to implement robtop's serialization for requests on
#[derive(Debug, Clone, Copy)]
struct LengthFilter(LevelLength);

impl Serialize for LengthFilter {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let numerical_value = match self.0 {
            LevelLength::Unknown(unknown) => unknown,
            LevelLength::Tiny => 0,
            LevelLength::Short => 1,
            LevelLength::Medium => 2,
            LevelLength::Long => 3,
            LevelLength::ExtraLong => 4,
        };

        serializer.serialize_i32(numerical_value)
    }
}

/// Newtype struct for [`LevelRating`] to implement robtop's serialization for requests on
#[derive(Debug, Clone, Copy)]
struct RatingFilter(LevelRating);

impl Serialize for RatingFilter {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let numerical_value = match self.0 {
            LevelRating::Unknown(value) => value,
            LevelRating::NotAvailable => -1,
            LevelRating::Auto => -3,
            LevelRating::Easy => 1,
            LevelRating::Normal => 2,
            LevelRating::Hard => 3,
            LevelRating::Harder => 4,
            LevelRating::Insane => 5,
            LevelRating::Demon(_) => -2, /* The value doesn't matter, since setting the request field "rating" to
                                          * -2 means "search for any demon, regardless of difficulty" */
        };

        serializer.serialize_i32(numerical_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::level::LevelLength,
        request::level::{CompletionFilter, LevelRequestType, LevelsRequest, SearchFilters},
        serde::RequestSerializer,
    };
    use serde::Serialize;

    #[test]
    fn serialize_levels_request() {
        let request =
            LevelsRequest::default()
                .request_type(LevelRequestType::MostLiked)
                .with_length(LevelLength::Medium)
                .with_length(LevelLength::Long)
                .search_filters(SearchFilters::default().featured().two_player().epic().rated().completion_filter(
                    CompletionFilter::exclude(vec![
                        18018958, 21373201, 22057275, 22488444, 22008823, 23144971, 17382902, 87600, 22031889, 22390740, 22243264, 21923305,
                    ]),
                ));

        let mut output = Vec::new();

        let mut serializer = RequestSerializer::new(&mut output);

        request.serialize(&mut serializer).unwrap();

        assert_eq!(
            std::str::from_utf8(&output),
            Ok(
                "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&type=2&str=&len=2,3&diff=-&page=0&total=0&featured=1&original=0&\
                 twoPlayer=1&coins=0&epic=1&star=1&completedLevels=(18018958,21373201,22057275,22488444,22008823,23144971,17382902,87600,\
                 22031889,22390740,22243264,21923305)&onlyCompleted=0&uncompleted=1"
            )
        );
    }
}
