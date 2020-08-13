use crate::model::user::{Color, ModLevel};
use serde::{export::Formatter, Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Youtube<'a>(Cow<'a, str>);

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Twitch<'a>(Cow<'a, str>);

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Twitter<'a>(Cow<'a, str>);

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
    /// The [`User`]'s name
    ///
    /// ## GD Internals:
    /// This value is provided at index `1`.
    pub name: Cow<'a, str>,

    /// The [`User`]'s unique user ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `2`
    pub user_id: u64,

    /// The amount of stars this [`User`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `3`
    pub stars: u32,

    /// The demons of stars this [`User`] has beaten.
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`
    pub demons: u16,

    /// The amount of creator points this [`User`] was awarded.
    ///
    /// ## GD Internals:
    /// This value is provided at index `8`
    pub creator_points: u16,

    /// This [`User`]'s primary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`. The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    pub primary_color: Color,

    /// This [`User`]'s secondary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `11`. Same things as above apply
    pub secondary_color: Color,

    /// The amount of secret coins this [`User`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `13`
    pub secret_coins: u8,

    /// The [`User`]'s unique account ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `16`
    pub account_id: u64,

    /// The amount of user coins this [`User`] has collected.
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

    /// The link to the [`User`]'s [YouTube](https://youtube.com) channel, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `20`. The value provided is only the `username` section of an `https://www.youtube.com/user/{username}` URL
    pub youtube_url: Option<Youtube<'a>>,

    /// The 1-based index of the cube this [`User`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `21`
    pub cube_index: u16,

    /// The 1-based index of the ship this [`User`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `22`
    pub ship_index: u8,

    /// The 1-based index of the ball this [`User`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `23`
    pub ball_index: u8,

    /// The 1-based index of the UFO this [`User`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `24`
    pub ufo_index: u8,

    /// The 1-based index of the wave this [`User`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `25`
    pub wave_index: u8,

    /// The 1-based index of the robot this [`User`] currently uses. Indexing of icons starts at the
    /// top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `26`
    pub robot_index: u8,

    /// Values indicating whether this [`User`] has glow activated or not.
    ///
    /// ## GD Internals:
    /// This value is provied at index `27`, as an integer
    pub has_glow: bool,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `29`
    pub index_29: Cow<'a, str>,

    /// This [`User`]'s global rank. [`None`] if he is banned or not ranked.
    ///
    /// ## GD Internals:
    /// This value is provided at index `30`. For unranked/banned users it's `0`
    pub global_rank: Option<u32>,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `31`
    pub index_31: Cow<'a, str>,

    /// The 1-based index of the spider this [`User`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `43`
    pub spider_index: u8,

    /// The link to the [`User`]'s [Twitter](https://twitter.com) account, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `44`. The value provided is only the `username` section of an `https://www.twitter.com/{username}` URL
    pub twitter_url: Option<Twitter<'a>>,

    /// The link to the [`User`]'s [Twitch](https://twitch.tv) channel, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `45`. The value provided is only the `username` section of an `https://twitch.tv/{username}` URL
    pub twitch_url: Option<Twitch<'a>>,

    /// The amount of diamonds this [`User`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `46`
    pub diamonds: u16,

    /// The 1-based index of the death-effect this [`User`] currently uses. Indexing of icons
    /// starts at the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provied at index `48`
    pub death_effect_index: u8,

    /// The level of moderator this [`User`] is
    ///
    /// ## GD Internals:
    /// This value is provided at index `49`
    pub mod_level: ModLevel,

    // TODO: figure this value out
    /// ## GD Internals:
    /// This value is provided at index `50`
    pub index_50: Cow<'a, str>,
}
