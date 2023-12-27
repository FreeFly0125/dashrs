use crate::{
    model::user::{Color, IconType},
    GJFormat,
};
use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use variant_partial_eq::VariantPartialEq;

/// Struct modelling the partial user data returned by the `getGJUsers` endpoint.
///
/// Note that no field `diamonds` exists here. This is consistent with Geometry Dash's behavior, as
/// the GD server exhibit a bug where they do not provide diamonds information, although the client
/// has the UI for it.
#[derive(Debug, Clone, VariantPartialEq, Eq, Serialize, Deserialize, Dash)]
pub struct SearchedUser<'a> {
    /// This [`SearchedUser`]'s name
    #[dash(index = 1)]
    pub name: Cow<'a, str>,

    /// The [`SearchedUser`]'s unique user ID
    #[dash(index = 2)]
    pub user_id: u64,

    /// This [`SearchedUser`]'s stars
    #[dash(index = 3)]
    pub stars: u32,

    /// This [`SearchedUser`]'s beaten demons
    #[dash(index = 4)]
    pub demons: u16,

    // TODO: figure this value out
    #[dash(index = 6)]
    pub index_6: Option<Cow<'a, str>>,

    /// This [`SearchedUser`]'s creator points
    #[dash(index = 8)]
    pub creator_points: u16,

    /// The index of the icon being displayed.
    #[dash(index = 9)]
    pub icon_index: u16,

    /// This [`SearchedUser`]'s primary color
    ///
    /// ## GD Internals:
    /// The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    #[dash(index = 10)]
    pub primary_color: Color,

    /// This [`SearchedUser`]'s secondary color
    ///
    /// ## GD Internals:
    /// Same things as above apply
    #[dash(index = 11)]
    pub secondary_color: Color,

    /// The amount of secret coins this [`SearchedUser`] has collected.
    #[dash(index = 13)]
    pub secret_coins: u8,

    /// The type of icon being displayed
    #[dash(index = 14)]
    pub icon_type: IconType,

    /// Values indicating whether this [`SearchedUser`] has glow activated or not.
    #[dash(index = 15)]
    #[dash(serialize_with = "crate::util::true_to_two")]
    pub has_glow: bool,

    /// The [`SearchedUser`]'s unique account ID
    #[dash(index = 16)]
    pub account_id: u64,

    /// The amount of user coins this [`SearchedUser`] has collected.
    #[dash(index = 17)]
    pub user_coins: u16,

    /// The number of moons this [`SearchedUser`] has collected. Currently always zero due to a
    /// server bug (similar to how the game always displays 0 diamonds here)
    #[dash(index = 52)]
    pub moons: u32,
}

impl<'de> GJFormat<'de> for SearchedUser<'de> {
    const DELIMITER: &'static str = ":";
    const MAP_LIKE: bool = true;
}
