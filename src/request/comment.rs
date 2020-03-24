//! Module containing request structs for retrieving profile/level comments

use crate::request::{BaseRequest, GD_21};
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// The different orderings that can be requested for level comments
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "u8")]
pub enum SortMode {
    /// Sort the comments by likes, in descending order
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `1` in the boomlings API
    Liked,

    /// Sort the comments from newest to oldest
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `0` in the boomlings API
    Recent,
}

impl Into<u8> for SortMode {
    fn into(self) -> u8 {
        match self {
            SortMode::Liked => 1,
            SortMode::Recent => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct LevelCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// What to sort by comments by
    ///
    /// ## GD Internals:
    /// This field is called `mode` in the boomlings API.
    #[serde(rename = "mode")]
    pub sort_mode: SortMode,

    /// The id of the level to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The amount of comments to retrieve. Note that while in-game this can only be set to 20 or 40
    /// (via the "load more comments option), the API accepts any value. So you can set it to
    /// something ridiculously high (like u32::MAX_VALUE) and retrieve all comments at once.
    ///
    /// ## GD Internals:
    /// This field is called `count` in the boomlings API
    #[serde(rename = "count")]
    pub limit: u32,
}

impl LevelCommentsRequest<'_> {
    pub const fn new(level: u64) -> Self {
        LevelCommentsRequest {
            level_id: level,
            base: GD_21,
            page: 0,
            total: 0,
            sort_mode: SortMode::Recent,
            limit: 20,
        }
    }

    pub const fn liked(mut self) -> Self {
        self.sort_mode = SortMode::Liked;
        self
    }

    pub const fn recent(mut self) -> Self {
        self.sort_mode = SortMode::Recent;
        self
    }
}

impl Display for LevelCommentsRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "LevelCommentsRequest({})", self.level_id)
    }
}

impl<'a> Into<LevelCommentsRequest<'a>> for u64 {
    fn into(self) -> LevelCommentsRequest<'a> {
        LevelCommentsRequest::new(self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct ProfileCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// The account id of the user to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `accountID` in the boomlings API
    #[serde(rename = "accountID")]
    pub account_id: u64,
}

impl ProfileCommentsRequest<'_> {
    pub const fn new(account: u64) -> Self {
        ProfileCommentsRequest {
            account_id: account,
            base: GD_21,
            page: 0,
            total: 0,
        }
    }
}

impl Display for ProfileCommentsRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "AccountCommentsRequest({})", self.account_id)
    }
}
