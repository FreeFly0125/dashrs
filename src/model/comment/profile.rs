use crate::{
    serde::{Base64Decoder, Thunk},
    GJFormat,
};
use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use variant_partial_eq::VariantPartialEq;

#[derive(Debug, Serialize, Deserialize, Eq, VariantPartialEq, Clone, Dash)]
pub struct ProfileComment<'a> {
    /// The actual content of the [`ProfileComment`] made.
    #[serde(borrow)]
    #[variant_compare = "crate::util::option_variant_eq"]
    #[dash(index = 2)]
    pub content: Option<Thunk<'a, Base64Decoder>>,

    /// The amount of likes this [`ProfileComment`] has received
    #[dash(index = 4)]
    pub likes: i32,

    /// The unique id of this [`ProfileComment`]. Additionally, there is also no [`LevelComment`](crate::model::comment::level::LevelComment)
    /// with this id
    #[dash(index = 6)]
    pub comment_id: u64,

    /// Robtop's completely braindead way of keeping track of when this [`ProfileComment`] was
    /// posted
    #[dash(index = 9)]
    pub time_since_post: Cow<'a, str>,
}

impl<'de> GJFormat<'de> for ProfileComment<'de> {
    const DELIMITER: &'static str = "~";
    const MAP_LIKE: bool = true;
}
