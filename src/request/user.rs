//! Module containing request definitions for retrieving users

use crate::{
    model::creator::Creator,
    request::{BaseRequest, GD_21, REQUEST_BASE_URL},
};
use serde::Serialize;
use std::fmt::{Display, Error, Formatter};

pub const GET_USER_ENDPOINT: &str = "getGJUserInfo20";
pub const SEARCH_USER_ENDPOINT: &str = "getGJUsers20";

/// Struct modelled after a request to `getGJUserInfo20.php`.
///
/// In the geometry Dash API, this endpoint is used to download player profiles from the servers by
/// their account IDs
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct UserRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The **account ID** (_not_ user ID) of the users whose data to retrieve.
    ///
    /// ## GD Internals:
    /// This field is called `targetAccountID` in the boomlings API
    #[serde(rename = "targetAccountID")]
    pub user: u64,
}

impl UserRequest<'_> {
    pub const fn new(user_id: u64) -> UserRequest<'static> {
        UserRequest {
            base: GD_21,
            user: user_id,
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}{}", REQUEST_BASE_URL, super::to_string(self), GET_USER_ENDPOINT)
    }
}

impl From<u64> for UserRequest<'_> {
    fn from(user_id: u64) -> Self {
        UserRequest::new(user_id)
    }
}

impl From<Creator<'_>> for UserRequest<'_> {
    fn from(creator: Creator<'_>) -> Self {
        UserRequest::from(creator.user_id)
    }
}

impl Display for UserRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "UserRequest({})", self.user)
    }
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq)]
pub struct UserSearchRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve
    ///
    /// Since the behavior of the search function was changed to return only the user whose name
    /// matches the search string exactly (previous behavior was a prefix search), it is not
    /// possible to retrieve more than 1 user via this endpoint anymore, rendering the pagination
    /// parameters useless.
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// The name of the user being searched for
    ///
    /// ## GD Internals:
    /// This field is called `str` in the boomlings API
    #[serde(rename = "str")]
    pub search_string: &'a str,
}

impl<'a> UserSearchRequest<'a> {
    pub const fn new(search_string: &'a str) -> Self {
        UserSearchRequest {
            base: GD_21,
            total: 0,
            page: 0,
            search_string,
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}{}", REQUEST_BASE_URL, super::to_string(self), SEARCH_USER_ENDPOINT)
    }
}

impl<'a> From<&'a str> for UserSearchRequest<'a> {
    fn from(search_string: &'a str) -> Self {
        UserSearchRequest::new(search_string)
    }
}

impl<'a: 'b, 'b> From<&'b Creator<'a>> for UserSearchRequest<'b> {
    fn from(creator: &'b Creator<'a>) -> Self {
        UserSearchRequest::from(&*creator.name)
    }
}

impl Display for UserSearchRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "UserSearchRequest({})", self.search_string)
    }
}
