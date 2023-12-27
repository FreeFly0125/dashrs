use std::borrow::Cow;
// use std::borrow::Cow;

use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};
use variant_partial_eq::VariantPartialEq;

use crate::{
    model::user::{Color, IconType, ModLevel},
    serde::{Base64Decoder, Thunk},
    GJFormat, ProcessError, ThunkProcessor,
};

#[derive(Debug, Eq, VariantPartialEq, Clone, Deserialize, Serialize, Dash)]
pub struct LevelComment<'a> {
    /// Information about the user that made this [`LevelComment`]. Is generally a [`CommentUser`]
    /// object
    #[dash(no_index)]
    pub user: Option<CommentUser<'a>>,

    /// The actual content of the [`LevelComment`] made.
    #[dash(index = 2)]
    #[serde(borrow)]
    #[variant_compare = "crate::util::option_variant_eq"]
    pub content: Option<Thunk<'a, Base64Decoder>>,

    /// The unique user id of the player who made this [`LevelComment`]
    #[dash(index = 3)]
    pub user_id: u64,

    /// The amount of likes this [`LevelComment`] has received
    #[dash(index = 4)]
    pub likes: i32,

    /// The unique id of this [`LevelComment`]. Additionally, there is also no [`ProfileComment`](crate::model::comment::profile::ProfileComment)
    /// with this id
    #[dash(index = 6)]
    pub comment_id: u64,

    /// Whether this [`LevelComment`] has been flagged as spam (because of having received too many
    /// dislikes or for other reasons)
    #[dash(index = 7)]
    pub is_flagged_spam: bool,

    /// Robtop's completely braindead way of keeping track of when this [`LevelComment`] was posted
    #[dash(index = 9)]
    pub time_since_post: Cow<'a, str>,

    /// If enabled by the user making this [`LevelComment`], the progress they have done on the
    /// level this comment is on.
    #[dash(index = 10)]
    pub progress: Option<u8>,

    /// The level of moderator the player that made this [`LevelComment`] is
    #[dash(index = 11)]
    pub mod_level: ModLevel,

    /// If this [`LevelComment`]'s text is displayed in a special color (blue for robtop, green for
    /// elder mods), the RGB code of that color will be stored here
    ///
    /// Note that the yellow color of comments made by the creator is not reported here.
    #[dash(index = 12)]
    #[variant_compare = "crate::util::option_variant_eq"]
    pub special_color: Option<Thunk<'a, Color>>,
}

impl<'de> GJFormat<'de> for LevelComment<'de> {
    const DELIMITER: &'static str = "~";
    const MAP_LIKE: bool = true;
}

impl ThunkProcessor for Color {
    type Error = ProcessError;
    type Output<'a> = Color;

    fn from_unprocessed(unprocessed: Cow<str>) -> Result<Self::Output<'_>, Self::Error> {
        let mut split = unprocessed.split(',');

        let r = split.next();
        let g = split.next();
        let b = split.next();

        if split.next().is_some() {
            return Err(ProcessError::IncorrectLength { expected: 3 });
        }

        match (r, g, b) {
            (Some(r), Some(g), Some(b)) => Ok(Color::Known(r.parse()?, g.parse()?, b.parse()?)),
            _ => Err(ProcessError::IncorrectLength { expected: 3 }),
        }
    }

    fn as_unprocessed<'b>(processed: &'b Self::Output<'_>) -> Result<Cow<'b, str>, Self::Error> {
        match processed {
            Color::Known(r, g, b) => Ok(Cow::Owned(format!("{},{},{}", r, g, b))),
            _ => Err(ProcessError::Unrepresentable),
        }
    }

    fn downcast_output_lifetime<'b: 'c, 'c, 's>(output: &'s Self::Output<'b>) -> &'s Self::Output<'c> {
        output
    }
}

#[derive(Debug, Eq, VariantPartialEq, Clone, Deserialize, Serialize, Dash)]
pub struct CommentUser<'a> {
    /// This [`CommentUser`]'s name
    #[dash(index = 1)]
    pub name: Cow<'a, str>,

    /// The index of the icon being displayed.
    #[dash(index = 9)]
    pub icon_index: u16,

    /// This [`CommentUser`]'s primary color
    ///
    /// ## GD Internals:
    /// The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    #[dash(index = 10)]
    pub primary_color: Color,

    /// This [`CommentUser`]'s secondary color
    #[dash(index = 11)]
    pub secondary_color: Color,

    /// The type of icon being displayed
    #[dash(index = 14)]
    pub icon_type: IconType,

    /// Values indicating whether this [`CommentUser`] has glow activated or not.
    #[dash(index = 15)]
    #[dash(serialize_with = "crate::util::true_to_two")]
    pub has_glow: bool,

    /// The [`CommentUser`]'s unique account ID
    #[dash(index = 16)]
    pub account_id: Option<u64>,
}

impl<'de> GJFormat<'de> for CommentUser<'de> {
    const DELIMITER: &'static str = "~";
    const MAP_LIKE: bool = true;
}
