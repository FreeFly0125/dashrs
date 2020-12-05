use crate::{Base64Decoded, Thunk};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ProfileComment<'a> {
    /// The actual content of the [`ProfileComment`] made.
    ///
    /// ## GD Internals
    /// This value is provided at index `2` and base64 encoded
    #[serde(borrow)]
    pub content: Option<Thunk<'a, Base64Decoded<'a>>>,

    /// The amount of likes this [`ProfileComment`] has received
    ///
    /// ## GD Internals
    /// This value is provided at index `4`
    pub likes: i32,

    /// The unique id of this [`ProfileComment`]. Additionally, there is also no [`LevelComment`]
    /// with this idea
    ///
    /// ## GD Internals
    /// This value is provided at index `6`
    pub comment_id: u64,

    /// Robtop's completely braindead way of keeping track of when this [`ProfileComment`] was
    /// posted
    ///
    /// ## GD Internals
    /// This value is provided at index `9`
    pub time_since_post: Cow<'a, str>,
}

mod internal {
    use crate::model::comment::profile::ProfileComment;

    include!(concat!(env!("OUT_DIR"), "/profile_comment.boilerplate"));
}
