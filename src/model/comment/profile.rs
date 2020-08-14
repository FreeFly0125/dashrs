use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ProfileComment<'a> {
    /// The actual content of the [`ProfileComment`] made.
    ///
    /// ## GD Internals
    /// This value is provided at index `2`
    pub content: Option<Cow<'a, str>>,

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
    use crate::{
        model::comment::profile::ProfileComment,
        serde::{IndexedDeserializer, IndexedSerializer},
        DeError, HasRobtopFormat, SerError,
    };
    use serde::{Deserialize, Serialize};
    use std::{
        borrow::{Borrow, Cow},
        io::Write,
    };

    #[derive(Serialize, Deserialize)]
    struct InternalProfileComment<'a> {
        #[serde(rename = "2")]
        pub content: Option<&'a str>,

        #[serde(rename = "4")]
        pub likes: i32,

        #[serde(rename = "6")]
        pub comment_id: u64,

        #[serde(rename = "9")]
        pub time_since_post: &'a str,
    }

    impl<'a> HasRobtopFormat<'a> for ProfileComment<'a> {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError<'a>> {
            let internal = InternalProfileComment::deserialize(&mut IndexedDeserializer::new(input, "~", true))?;

            Ok(ProfileComment {
                content: internal.content.map(Cow::Borrowed),
                likes: internal.likes,
                comment_id: internal.comment_id,
                time_since_post: Cow::Borrowed(internal.time_since_post),
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalProfileComment {
                content: self.content.as_ref().map(Borrow::borrow),
                likes: self.likes,
                comment_id: self.comment_id,
                time_since_post: self.time_since_post.borrow(),
            };

            internal.serialize(&mut IndexedSerializer::new("~", writer, true))
        }
    }
}
