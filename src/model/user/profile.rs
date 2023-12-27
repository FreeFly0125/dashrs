use crate::{
    model::user::{Color, ModLevel},
    GJFormat,
};
use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};
use variant_partial_eq::VariantPartialEq;

crate::dash_rs_newtype!(Youtube);
crate::dash_rs_newtype!(Twitch);
crate::dash_rs_newtype!(Twitter);

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
#[derive(Debug, Eq, VariantPartialEq, Clone, Serialize, Deserialize, Dash)]
pub struct Profile<'a> {
    /// The [`Profile`]'s name
    #[dash(index = 1)]
    pub name: Cow<'a, str>,

    /// The [`Profile`]'s unique user ID
    #[dash(index = 2)]
    pub user_id: u64,

    /// The amount of stars this [`Profile`] has collected.
    #[dash(index = 3)]
    pub stars: u32,

    /// The demons of stars this [`Profile`] has beaten.
    #[dash(index = 4)]
    pub demons: u16,

    /// The amount of creator points this [`Profile`] was awarded.
    #[dash(index = 8)]
    pub creator_points: u16,

    /// This [`Profile`]'s primary color
    ///
    /// ## GD Internals:
    /// The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    #[dash(index = 10)]
    pub primary_color: Color,

    /// This [`Profile`]'s secondary color
    ///
    /// ## GD Internals:
    /// Same things as above apply
    #[dash(index = 11)]
    pub secondary_color: Color,

    /// The amount of secret coins this [`Profile`] has collected.
    #[dash(index = 13)]
    pub secret_coins: u8,

    /// The [`Profile`]'s unique account ID
    #[dash(index = 16)]
    pub account_id: u64,

    /// The amount of user coins this [`Profile`] has collected.
    #[dash(index = 17)]
    pub user_coins: u16,

    // TODO: figure this value out
    #[dash(index = 18)]
    pub index_18: Cow<'a, str>,

    // TODO: figure this value out
    #[dash(index = 19)]
    pub index_19: Cow<'a, str>,

    /// The link to the [`Profile`]'s [YouTube](https://youtube.com) channel, if provided
    ///
    /// ## GD Internals
    /// The value provided is only the `username` section of an `https://www.youtube.com/user/{username}` URL
    #[dash(index = 20)]
    pub youtube_url: Option<Youtube<'a>>,

    /// The 1-based index of the cube this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 21)]
    pub cube_index: u16,

    /// The 1-based index of the ship this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 22)]
    pub ship_index: u8,

    /// The 1-based index of the ball this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 23)]
    pub ball_index: u8,

    /// The 1-based index of the UFO this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 24)]
    pub ufo_index: u8,

    /// The 1-based index of the wave this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 25)]
    pub wave_index: u8,

    /// The 1-based index of the robot this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 26)]
    pub robot_index: u8,

    /// Values indicating whether this [`Profile`] has glow activated or not.
    #[dash(index = 28)]
    pub has_glow: bool,

    // TODO: figure this value out
    #[dash(index = 29)]
    pub index_29: Cow<'a, str>,

    /// This [`Profile`]'s global rank. [`None`] if he is banned or not ranked.
    ///
    /// ## GD Internals:
    /// For unranked/banned users it's `0`. TODO: Why is this an option?
    #[dash(index = 30)]
    pub global_rank: Option<u32>,

    // TODO: figure this value out
    #[dash(index = 31)]
    pub index_31: Cow<'a, str>,

    // TODO: figure this value out
    #[dash(index = 38)]
    #[dash(default)]
    #[dash(skip_serializing_if = "Option::is_none")]
    pub index_38: Option<Cow<'a, str>>,

    // TODO: figure this value out
    #[dash(index = 39)]
    #[dash(default)]
    #[dash(skip_serializing_if = "Option::is_none")]
    pub index_39: Option<Cow<'a, str>>,

    // TODO: figure this value out
    #[dash(index = 40)]
    #[dash(default)]
    #[dash(skip_serializing_if = "Option::is_none")]
    pub index_40: Option<Cow<'a, str>>,

    /// The 1-based index of the spider this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 43)]
    pub spider_index: u8,

    /// The link to the [`Profile`]'s [Twitter](https://twitter.com) account, if provided
    ///
    /// ## GD Internals
    /// The value provided is only the `username` section of an `https://www.twitter.com/{username}` URL
    #[dash(index = 44)]
    pub twitter_url: Option<Twitter<'a>>,

    /// The link to the [`Profile`]'s [Twitch](https://twitch.tv) channel, if provided
    ///
    /// ## GD Internals
    /// The value provided is only the `username` section of an `https://twitch.tv/{username}` URL
    #[dash(index = 45)]
    pub twitch_url: Option<Twitch<'a>>,

    /// The amount of diamonds this [`Profile`] has collected.
    #[dash(index = 46)]
    pub diamonds: u16,

    /// The 1-based index of the death-effect this [`Profile`] currently uses. Indexing of icons
    /// starts at the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 48)]
    pub death_effect_index: u8,

    /// The level of moderator this [`Profile`] is
    #[dash(index = 49)]
    pub mod_level: ModLevel,

    // TODO: figure this value out
    #[dash(index = 50)]
    pub index_50: Cow<'a, str>,

    #[dash(index = 51)]
    pub index_51: Cow<'a, str>,

    /// The number of moons this [`Profile`] has collected
    #[dash(index = 52)]
    pub moons: u32,

    /// The 1-based index of the swing this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 53)]
    pub swing_index: u8,

    /// The 1-based index of the jetpack this [`Profile`] currently uses. Indexing of icons starts
    /// at the top left corner and then goes left-to-right and top-to-bottom
    #[dash(index = 54)]
    pub jetpack_index: u8,
}

impl<'de> GJFormat<'de> for Profile<'de> {
    const DELIMITER: &'static str = ":";
    const MAP_LIKE: bool = true;
}
