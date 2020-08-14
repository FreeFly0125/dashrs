use std::borrow::Cow;
use crate::model::user::{Color, IconType};
use serde::{Deserialize, Serialize};
use crate::{Thunk, Base64Decoded};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct LevelComment<'a> {
    /// Information about the user that made this [`LevelComment`]. Is generally a [`CommentUser`]
    /// object
    pub user: Option<CommentUser<'a>>,

    /// The actual content of the [`LevelComment`] made.
    ///
    /// ## GD Internals
    /// This value is provided at index `2`
    pub content: Option<Thunk<'a, Base64Decoded<'a>>>,

    /// The unique user id of the player who made this [`LevelComment`]
    ///
    /// ## GD Internals
    /// This value is provided at index `3`
    pub user_id: Cow<'a, str>,

    /// The amount of likes this [`LevelComment`] has received
    ///
    /// ## GD Internals
    /// This value is provided at index `4`
    pub likes: i32,

    /// The unique id of this [`LevelComment`]. Additionally, there is also no [`ProfileComment`]
    /// with this idea
    ///
    /// ## GD Internals
    /// This value is provided at index `6`
    pub comment_id: u64,

    /// Whether this [`LevelComment`] has been flagged as spam (because of having received too many
    /// dislikes or for other reasons)
    ///
    /// ## GD Internals
    /// This value is provided at index `7`
    pub is_flagged_spam: bool,

    /// Robtop's completely braindead way of keeping track of when this [`LevelComment`] was posted
    ///
    /// ## GD Internals
    /// This value is provided at index `9`
    pub time_since_post: Cow<'a, str>,

    /// If enabled by the user making this [`LevelComment`], the progress they have done on the
    /// level this comment is on.
    ///
    /// ## GD Internals
    /// This value is provided at index `10`
    pub progress: Option<u8>,

    /// Whether the player that made this [`LevelComment`] is an elder mod
    ///
    /// ## GD Internals
    /// This value is provided at index `11`
    pub is_elder_mod: bool,

    /// If this [`LevelComment`]'s text is displayed in a special color (blue for robtop, green for
    /// elder mods), the RGB code of that color will be stored here
    ///
    /// Note that the yellow color of comments made by the creator is not reported here.
    ///
    /// ## GD Internals
    /// This value is provided at index `12`
    pub special_color: Option<Color>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct CommentUser<'a> {
    /// This [`CommentUser`]'s name
    ///
    /// ## GD Internals
    /// This value is provided at index `1`
    pub name: Cow<'a, str>,

    /// The index of the icon being displayed.
    ///
    /// ## GD Internals
    /// This value is provided at index `9`
    pub icon_index: u16,

    /// This [`CommentUser`]'s primary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`. The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    /// ## GD Internals
    /// This value is provided at index `10`
    pub primary_color: Color,

    /// This [`CommentUser`]'s secondary color
    ///
    /// ## GD Internals
    /// This value is provided at index `11`. Same things as above apply
    pub secondary_color: Color,

    /// The type of icon being displayed
    ///
    /// ## GD Internals
    /// This value is provided at index `14`
    pub icon_type: IconType,

    /// Values indicating whether this [`CommentUser`] has glow activated or not.
    ///
    /// ## GD Internals
    /// This value is provided at index `15`
    pub has_glow: bool,

    /// The [`CommentUser`]'s unique account ID
    ///
    /// ## GD Internals
    /// This value is provided at index `16`
    pub account_id: Option<u64>,
}
