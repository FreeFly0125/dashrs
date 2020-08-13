use crate::model::user::{Color, ModLevel};
use serde::{export::Formatter, Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Youtube<'a>(pub Cow<'a, str>);

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Twitch<'a>(pub Cow<'a, str>);

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Twitter<'a>(pub Cow<'a, str>);

impl Display for Youtube<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.youtube.com/channel/{}", self.0)
    }
}

impl Display for Twitch<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.twitch.tv/{}", self.0)
    }
}

impl Display for Twitter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.twitter.com/{}", self.0)
    }
}

/// Struct representing a Geometry Dash User's profile, as seen after clicking their name in the
/// official client
///
/// ## GD Internals:
/// The Geometry Dash servers provide user data in a `getGJUserInfo` response
///
/// ### Unused Indices
/// The following indices aren't used by the Geometry Dash servers: `5`, `6`, `7`, `9`, `12`, `14`,
/// `15`, `27`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `47`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Profile<'a> {
    /// The [`Profile`]'s name
    ///
    /// ## GD Internals:
    /// This value is provided at index `1`.
    pub name: Cow<'a, str>,

    /// The [`Profile`]'s unique user ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `2`
    pub user_id: u64,

    /// The amount of stars this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `3`
    pub stars: u32,

    /// The demons of stars this [`Profile`] has beaten.
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`
    pub demons: u16,

    /// The amount of creator points this [`Profile`] was awarded.
    ///
    /// ## GD Internals:
    /// This value is provided at index `8`
    pub creator_points: u16,

    /// This [`Profile`]'s primary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`. The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    pub primary_color: Color,

    /// This [`Profile`]'s secondary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `11`. Same things as above apply
    pub secondary_color: Color,

    /// The amount of secret coins this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `13`
    pub secret_coins: u8,

    /// The [`Profile`]'s unique account ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `16`
    pub account_id: u64,

    /// The amount of user coins this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `17`
    pub user_coins: u16,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `18`
    pub index_18: Cow<'a, str>,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `19`
    pub index_19: Cow<'a, str>,

    /// The link to the [`Profile`]'s [YouTube](https://youtube.com) channel, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `20`. The value provided is only the `username` section of an `https://www.youtube.com/user/{username}` URL
    pub youtube_url: Option<Youtube<'a>>,

    /// The 1-based index of the cube this [`Profile`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `21`
    pub cube_index: u16,

    /// The 1-based index of the ship this [`Profile`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `22`
    pub ship_index: u8,

    /// The 1-based index of the ball this [`Profile`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `23`
    pub ball_index: u8,

    /// The 1-based index of the UFO this [`Profile`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `24`
    pub ufo_index: u8,

    /// The 1-based index of the wave this [`Profile`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `25`
    pub wave_index: u8,

    /// The 1-based index of the robot this [`Profile`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `26`
    pub robot_index: u8,

    /// Values indicating whether this [`Profile`] has glow activated or not.
    ///
    /// ## GD Internals:
    /// This value is provided at index `28`, as an integer
    pub has_glow: bool,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `29`
    pub index_29: Cow<'a, str>,

    /// This [`Profile`]'s global rank. [`None`] if he is banned or not ranked.
    ///
    /// ## GD Internals:
    /// This value is provided at index `30`. For unranked/banned users it's `0`
    pub global_rank: Option<u32>,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `31`
    pub index_31: Cow<'a, str>,

    /// The 1-based index of the spider this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `43`
    pub spider_index: u8,

    /// The link to the [`Profile`]'s [Twitter](https://twitter.com) account, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `44`. The value provided is only the `username` section of an `https://www.twitter.com/{username}` URL
    pub twitter_url: Option<Twitter<'a>>,

    /// The link to the [`Profile`]'s [Twitch](https://twitch.tv) channel, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `45`. The value provided is only the `username` section of an `https://twitch.tv/{username}` URL
    pub twitch_url: Option<Twitch<'a>>,

    /// The amount of diamonds this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `46`
    pub diamonds: u16,

    /// The 1-based index of the death-effect this [`Profile`] currently uses. Indexing of icons
    /// starts at the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `48`
    pub death_effect_index: u8,

    /// The level of moderator this [`Profile`] is
    ///
    /// ## GD Internals:
    /// This value is provided at index `49`
    pub mod_level: ModLevel,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `50`
    pub index_50: Cow<'a, str>,
}

mod internal {
    use crate::{
        model::user::{
            profile::{Profile, Twitch, Twitter, Youtube},
            Color, ModLevel,
        },
        serde::{IndexedDeserializer, IndexedSerializer},
        DeError, HasRobtopFormat, SerError,
    };
    use serde::{Deserialize, Serialize};
    use std::{
        borrow::{Borrow, Cow},
        io::Write,
    };

    #[derive(Serialize, Deserialize)]
    struct InternalProfile<'a> {
        #[serde(rename = "1")]
        pub name: &'a str,

        #[serde(rename = "2")]
        pub user_id: u64,

        #[serde(rename = "3")]
        pub stars: u32,

        #[serde(rename = "4")]
        pub demons: u16,

        #[serde(rename = "8")]
        pub creator_points: u16,

        #[serde(rename = "10")]
        pub primary_color: u8,

        #[serde(rename = "11")]
        pub secondary_color: u8,

        #[serde(rename = "13")]
        pub secret_coins: u8,

        #[serde(rename = "16")]
        pub account_id: u64,

        #[serde(rename = "17")]
        pub user_coins: u16,

        #[serde(rename = "18")]
        pub index_18: &'a str,

        #[serde(rename = "19")]
        pub index_19: &'a str,

        #[serde(rename = "20")]
        pub youtube_url: Option<&'a str>,

        #[serde(rename = "21")]
        pub cube_index: u16,

        #[serde(rename = "22")]
        pub ship_index: u8,

        #[serde(rename = "23")]
        pub ball_index: u8,

        #[serde(rename = "24")]
        pub ufo_index: u8,

        #[serde(rename = "25")]
        pub wave_index: u8,

        #[serde(rename = "26")]
        pub robot_index: u8,

        #[serde(rename = "28")]
        pub has_glow: bool,

        #[serde(rename = "29")]
        pub index_29: &'a str,

        #[serde(rename = "30")]
        pub global_rank: Option<u32>,

        #[serde(rename = "31")]
        pub index_31: &'a str,

        #[serde(rename = "43")]
        pub spider_index: u8,

        #[serde(rename = "44")]
        pub twitter_url: Option<&'a str>,

        #[serde(rename = "45")]
        pub twitch_url: Option<&'a str>,

        #[serde(rename = "46")]
        pub diamonds: u16,

        #[serde(rename = "48")]
        pub death_effect_index: u8,

        #[serde(rename = "49")]
        pub mod_level: u8,

        #[serde(rename = "50")]
        pub index_50: &'a str,
    }

    impl<'a> HasRobtopFormat<'a> for Profile<'a> {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError<'a>> {
            let internal = InternalProfile::deserialize(&mut IndexedDeserializer::new(input, ":", true))?;

            Ok(Profile {
                name: Cow::Borrowed(internal.name),
                user_id: internal.user_id,
                stars: internal.stars,
                demons: internal.demons,
                creator_points: internal.creator_points,
                primary_color: Color::from(internal.primary_color),
                secondary_color: Color::from(internal.secondary_color),
                secret_coins: internal.secret_coins,
                account_id: internal.account_id,
                user_coins: internal.user_coins,
                index_18: Cow::Borrowed(internal.index_18),
                index_19: Cow::Borrowed(internal.index_19),
                youtube_url: internal.youtube_url.map(Cow::Borrowed).map(Youtube),
                cube_index: internal.cube_index,
                ship_index: internal.ship_index,
                ball_index: internal.ball_index,
                ufo_index: internal.ufo_index,
                wave_index: internal.wave_index,
                robot_index: internal.robot_index,
                has_glow: internal.has_glow,
                index_29: Cow::Borrowed(internal.index_29),
                global_rank: internal.global_rank,
                index_31: Cow::Borrowed(internal.index_31),
                spider_index: internal.spider_index,
                twitter_url: internal.twitter_url.map(Cow::Borrowed).map(Twitter),
                twitch_url: internal.twitch_url.map(Cow::Borrowed).map(Twitch),
                diamonds: internal.diamonds,
                death_effect_index: internal.death_effect_index,
                mod_level: ModLevel::from(internal.mod_level),
                index_50: Cow::Borrowed(internal.index_50),
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalProfile {
                name: self.name.borrow(),
                user_id: self.user_id,
                stars: self.stars,
                demons: self.demons,
                creator_points: self.creator_points,
                primary_color: self.primary_color.into(),
                secondary_color: self.secondary_color.into(),
                secret_coins: self.secret_coins,
                account_id: self.account_id,
                user_coins: self.user_coins,
                index_18: self.index_18.borrow(),
                index_19: self.index_19.borrow(),
                youtube_url: self.youtube_url.as_ref().map(|y| y.0.borrow()),
                cube_index: self.cube_index,
                ship_index: self.ship_index,
                ball_index: self.ball_index,
                ufo_index: self.ufo_index,
                wave_index: self.wave_index,
                robot_index: self.robot_index,
                has_glow: self.has_glow,
                index_29: self.index_29.borrow(),
                global_rank: self.global_rank,
                index_31: self.index_31.borrow(),
                spider_index: self.spider_index,
                twitter_url: self.twitter_url.as_ref().map(|t| t.0.borrow()),
                twitch_url: self.twitch_url.as_ref().map(|t| t.0.borrow()),
                diamonds: self.diamonds,
                death_effect_index: self.death_effect_index,
                mod_level: self.mod_level.into(),
                index_50: self.index_50.borrow(),
            };

            internal.serialize(&mut IndexedSerializer::new(":", writer, true))
        }
    }
}
