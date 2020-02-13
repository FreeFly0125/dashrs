use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod internal {
    use crate::{
        model::creator::Creator,
        serde::{HasRobtopFormat, ShortenLifetime},
    };
    use std::borrow::Cow;

    impl<'a> HasRobtopFormat<'a> for Creator<'a> {
        type Internal = Creator<'a>;

        const DELIMITER: &'static str = ":";
        const MAP_LIKE: bool = false;

        fn into_internal(self) -> Self::Internal {
            self
        }

        fn from_internal(int: Self::Internal) -> Self {
            int
        }
    }

    impl<'b, 'a> ShortenLifetime<'b> for Creator<'a> {
        type Shortened = Creator<'b>;

        fn shorten(&'b self) -> Self::Shortened {
            Creator {
                name: Cow::Borrowed(self.name.as_ref()),
                ..*self
            }
        }
    }
}

/// Struct modelling a [`Creator`] of a level.
///
/// ## GD Internals
/// The Geometry Dash servers provide a list of the creators of the
/// levels in a `getGJLevels` response.
///
/// Creators do not use the map-like representation, meaning the order of fields in the raw data
/// must correspond to the order of fields in this struct.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Creator<'a> {
    /// The [`Creator`]'s unique user ID
    pub user_id: u64,

    /// The [`Creator`]'s name
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    /// The [`Creator`]'s unique account ID.
    ///
    /// This field is [`None`] if the creator hasn't registered for an account.
    pub account_id: Option<u64>,
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
