use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::model::creator::Creator;

/// Struct modelling a [`Creator`] the way it is represented by the Geometry Dash servers
///
/// See [`Creator`] for an owned version.
///
/// The Geometry Dash servers provide a list of the creators of the
/// levels in a `getGJLevels` response.
///
/// Creators do not use the map-like representation, meaning the order of fields in the raw data
/// must correspond to the order of fields in this struct.
#[derive(Debug, Deserialize, Serialize)]
pub struct RawCreator<'a> {
    pub user_id: u64,

    #[serde(borrow)]
    pub name: Cow<'a, str>,

    pub account_id: Option<u64>,
}


impl<'a> RawCreator<'a> {
    pub fn to_owned(self) -> Creator {
        Creator {
            user_id: self.user_id,
            name: self.name.into_owned(),
            account_id: self.account_id
        }
    }
}

impl Creator {
    pub fn as_raw(&self) -> RawCreator {
        RawCreator {
            user_id: self.user_id,
            name: Cow::Borrowed(self.name.as_ref()),
            account_id: self.account_id,
        }
    }
}
