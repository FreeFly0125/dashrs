use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use variant_partial_eq::VariantPartialEq;

use crate::GJFormat;

/// Struct modelling a [`Creator`] of a level.
///
/// ## GD Internals
/// The Geometry Dash servers provide a list of the creators of the
/// levels in a `getGJLevels` response.
///
/// Creators do not use the map-like representation, meaning the order of fields in the raw data
/// must correspond to the order of fields in this struct.
#[derive(Debug, Deserialize, Serialize, VariantPartialEq, Eq, Clone, Dash)]
pub struct Creator<'a> {
    /// The [`Creator`]'s unique user ID
    #[dash(index = 1)]
    pub user_id: u64,

    /// The [`Creator`]'s name
    #[serde(borrow)]
    #[dash(index = 2)]
    pub name: Cow<'a, str>,

    /// The [`Creator`]'s unique account ID.
    ///
    /// This field is [`None`] if the creator hasn't registered for an account.
    #[dash(index = 3)]
    #[dash(with = "crate::util::default_to_none")]
    pub account_id: Option<u64>,
}

impl<'de> GJFormat<'de> for Creator<'de> {
    const DELIMITER: &'static str = ":";
    const MAP_LIKE: bool = false;
}

impl<'a> Creator<'a> {
    pub fn into_owned(self) -> Creator<'static> {
        Creator {
            user_id: self.user_id,
            name: Cow::Owned(self.name.into_owned()),
            account_id: self.account_id,
        }
    }
}
