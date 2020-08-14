use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{
    model::user::{Color, IconType},
    Base64Decoded, Thunk,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct LevelComment<'a> {
    /// Information about the user that made this [`LevelComment`]. Is generally a [`CommentUser`]
    /// object
    pub user: Option<CommentUser<'a>>,

    /// The actual content of the [`LevelComment`] made.
    ///
    /// ## GD Internals
    /// This value is provided at index `2` and is base64 encoded
    #[serde(borrow)]
    pub content: Option<Thunk<'a, Base64Decoded<'a>>>,

    /// The unique user id of the player who made this [`LevelComment`]
    ///
    /// ## GD Internals
    /// This value is provided at index `3`
    pub user_id: u64,

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
    /// This value is provided at index `11`, however the value `true` is encoded as `"2"` instead
    /// of `"1"`
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
    /// This value is provided at index `15`, however the value `true` is encoded as `"2"` instead
    pub has_glow: bool,

    /// The [`CommentUser`]'s unique account ID
    ///
    /// ## GD Internals
    /// This value is provided at index `16`
    pub account_id: Option<u64>,
}

mod internal {
    use std::borrow::{Borrow, Cow};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::{
        model::{comment::level::LevelComment, user::Color},
        serde::{IndexedDeserializer, IndexedSerializer, Internal},
        Base64Decoded, DeError, HasRobtopFormat, SerError, Thunk,
    };
    use std::io::Write;
    use crate::model::comment::level::CommentUser;

    struct RGBColor(u8, u8, u8);

    impl Serialize for RGBColor {
        fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
            where
                S: Serializer,
        {
            serializer.serialize_str(&format!("{},{},{}", self.0, self.1, self.2))
        }
    }

    impl<'de> Deserialize<'de> for RGBColor {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
            where
                D: Deserializer<'de>,
        {
            let color_string = <&str>::deserialize(deserializer)?;
            let mut split = color_string.split(',');

            if let (Some(r), Some(g), Some(b)) = (split.next(), split.next(), split.next()) {
                Ok(RGBColor(
                    r.parse().map_err(serde::de::Error::custom)?,
                    g.parse().map_err(serde::de::Error::custom)?,
                    b.parse().map_err(serde::de::Error::custom)?,
                ))
            } else {
                Err(serde::de::Error::custom(format!("Malformed color string {}", color_string)))
            }
        }
    }

    #[derive(Deserialize, Serialize)]
    struct InternalLevelComment<'a> {
        #[serde(rename = "2")]
        pub content: Option<Internal<Thunk<'a, Base64Decoded<'a>>>>,

        #[serde(rename = "3")]
        pub user_id: u64,

        #[serde(rename = "4")]
        pub likes: i32,

        #[serde(rename = "6")]
        pub comment_id: u64,

        #[serde(rename = "7")]
        pub is_flagged_spam: bool,

        #[serde(rename = "9")]
        pub time_since_post: &'a str,

        #[serde(rename = "10")]
        pub progress: Option<u8>,

        #[serde(rename = "11", with = "crate::util::two_bool")]
        pub is_elder_mod: bool,

        #[serde(rename = "12")]
        pub special_color: Option<RGBColor>,
    }

    impl<'a> HasRobtopFormat<'a> for LevelComment<'a> {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError<'a>> {
            let internal = InternalLevelComment::deserialize(&mut IndexedDeserializer::new(input, "~", true))?;

            Ok(LevelComment {
                user: None,
                content: internal.content.map(|i| i.0),
                user_id: internal.user_id,
                likes: internal.likes,
                comment_id: internal.comment_id,
                is_flagged_spam: internal.is_flagged_spam,
                time_since_post: Cow::Borrowed(internal.time_since_post),
                progress: internal.progress,
                is_elder_mod: internal.is_elder_mod,
                special_color: internal.special_color.map(|RGBColor(r, g, b)| Color::Known(r, g, b)),
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalLevelComment {
                content: self.content.as_ref().map(|thunk| {
                    Internal(match thunk {
                        Thunk::Unprocessed(unproc) => Thunk::Unprocessed(unproc),
                        Thunk::Processed(Base64Decoded(moo)) => Thunk::Processed(Base64Decoded(Cow::Borrowed(moo.borrow()))),
                    })
                }),
                user_id: self.user_id,
                likes: self.likes,
                comment_id: self.comment_id,
                is_flagged_spam: self.is_flagged_spam,
                time_since_post: self.time_since_post.borrow(),
                progress: self.progress,
                is_elder_mod: self.is_elder_mod,
                special_color: self.special_color.map(|color| {
                    match color {
                        Color::Known(r, g, b) => RGBColor(r, g, b),
                        _ => panic!("Color::Unknown passed as color of level comment"),
                    }
                }),
            };

            internal.serialize(&mut IndexedSerializer::new("~", writer, true))
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct InternalCommentUser<'a> {
        #[serde(rename = "1")]
        pub name: &'a str,

        #[serde(rename = "9")]
        pub icon_index: u16,

        #[serde(rename = "10")]
        pub primary_color: u8,

        #[serde(rename = "11")]
        pub secondary_color: u8,

        #[serde(rename = "14")]
        pub icon_type: u8,

        #[serde(rename = "15", with = "crate::util::two_bool")]
        pub has_glow: bool,

        #[serde(rename = "16")]
        pub account_id: Option<u64>,
    }

    impl<'a> HasRobtopFormat<'a> for CommentUser<'a> {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError<'a>> {
            let internal = InternalCommentUser::deserialize(&mut IndexedDeserializer::new(input, "~", true))?;

            Ok(CommentUser {
                name: Cow::Borrowed(internal.name),
                icon_index: internal.icon_index,
                primary_color: internal.primary_color.into(),
                secondary_color: internal.secondary_color.into(),
                icon_type: internal.icon_type.into(),
                has_glow: internal.has_glow,
                account_id: internal.account_id
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalCommentUser {
                name: self.name.as_ref(),
                icon_index: self.icon_index,
                primary_color: self.primary_color.into(),
                secondary_color: self.secondary_color.into(),
                icon_type: self.icon_type.into(),
                has_glow: self.has_glow,
                account_id: self.account_id
            };

            internal.serialize(&mut IndexedSerializer::new("~", writer, true))
        }
    }

}
