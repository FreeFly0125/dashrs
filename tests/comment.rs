use dash_rs::{
    model::{
        comment::{
            level::{CommentUser, LevelComment},
            profile::ProfileComment,
        },
        user::{Color, IconType, ModLevel},
    },
    Base64Decoded, Thunk,
};
use std::borrow::Cow;

#[macro_use]
mod helper;

// A few of the most liked comments from "Ode to Time" by Pauze, 2020/08/15
const LEVEL_COMMENT1_DATA :&str = "2~U3BlY2lhbCB0aGFua3MgdG8gSGFkbywgQ2luY2ksIFN5bmFjdGl2ZSwgQ29vbCwgUHJpc20sIFN1Yndvb2ZlciwgYW5kIEhhZG8gZm9yIHBsYXl0ZXN0aW5nLg==~3~7226087~4~104~7~0~10~0~9~5 days~6~258976~11~2~12~75,255,75";
const LEVEL_COMMENT2_DATA: &str = "2~R3VydS4=~3~2723387~4~63~7~0~10~0~9~5 days~6~260007~11~2~12~75,255,75";
const LEVEL_COMMENT3_DATA: &str =
    "2~TGV0cyBtYWtlIGF1Z3VzdCAxMHRoIFBhdXplJ3MgaW50ZXJuYXRpb25hbCBkYXk=~3~7178197~4~58~7~0~10~0~9~5 days~6~259333~11~1~12~255,255,255";

const COMMENT_USER_DATA: &str = "1~Pauze~9~58~10~18~11~16~14~0~15~2~16~1705254";

const LEVEL_COMMENT1: LevelComment = LevelComment {
    user: None,
    content: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "Special thanks to Hado, Cinci, Synactive, Cool, Prism, Subwoofer, and Hado for playtesting.",
    )))),
    user_id: 7226087,
    likes: 104,
    comment_id: 258976,
    is_flagged_spam: false,
    time_since_post: Cow::Borrowed("5 days"),
    progress: Some(0),
    mod_level: ModLevel::Elder,
    special_color: Some(Color::Known(75, 255, 75)),
};

const LEVEL_COMMENT2: LevelComment = LevelComment {
    user: None,
    content: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed("Guru.")))),
    user_id: 2723387,
    likes: 63,
    comment_id: 260007,
    is_flagged_spam: false,
    time_since_post: Cow::Borrowed("5 days"),
    progress: Some(0),
    mod_level: ModLevel::Elder,
    special_color: Some(Color::Known(75, 255, 75)),
};

const LEVEL_COMMENT3: LevelComment = LevelComment {
    user: None,
    content: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "Lets make august 10th Pauze's international day",
    )))),
    user_id: 7178197,
    likes: 58,
    comment_id: 259333,
    is_flagged_spam: false,
    time_since_post: Cow::Borrowed("5 days"),
    progress: Some(0),
    mod_level: ModLevel::Normal,
    special_color: Some(Color::Known(255, 255, 255)),
};

const COMMENT_USER: CommentUser = CommentUser {
    name: Cow::Borrowed("Pauze"),
    icon_index: 58,
    primary_color: Color::Known(80, 80, 80),
    secondary_color: Color::Known(0, 200, 255),
    icon_type: IconType::Cube,
    has_glow: true,
    account_id: Some(1705254),
};

impl helper::ThunkProcessor for LevelComment<'_> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut cnt) = self.content {
            assert!(cnt.process().is_ok());
        }
    }
}

impl helper::ThunkProcessor for CommentUser<'_> {
    fn process_all_thunks(&mut self) {}
}

impl helper::ThunkProcessor for ProfileComment<'_> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut cnt) = self.content {
            assert!(cnt.process().is_ok())
        }
    }
}

load_save_roundtrip!(load_save_roundtrip1, LevelComment, LEVEL_COMMENT1_DATA, LEVEL_COMMENT1, "~", true);
load_save_roundtrip!(load_save_roundtrip2, LevelComment, LEVEL_COMMENT2_DATA, LEVEL_COMMENT2, "~", true);
load_save_roundtrip!(load_save_roundtrip3, LevelComment, LEVEL_COMMENT3_DATA, LEVEL_COMMENT3, "~", true);
load_save_roundtrip!(load_save_roundtrip_user, CommentUser, COMMENT_USER_DATA, COMMENT_USER, "~", true);

save_load_roundtrip!(save_load_roundtrip1, LevelComment, LEVEL_COMMENT1);
save_load_roundtrip!(save_load_roundtrip2, LevelComment, LEVEL_COMMENT2);
save_load_roundtrip!(save_load_roundtrip3, LevelComment, LEVEL_COMMENT3);
save_load_roundtrip!(save_load_roundtrip_user, CommentUser, COMMENT_USER);

const PROFILE_COMMENT_DATA: &str =
    "2~QSB3aW5kb3cgdG8gdGhlIHBhc3QsIGEgZ2xpbXBzZSBvZiB0aGUgZnV0dXJlLCBBbiBPZGUgdG8gVGltZS4=~4~432~9~6 days~6~1922667";

const PROFILE_COMMENT: ProfileComment = ProfileComment {
    content: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "A window to the past, a glimpse of the future, An Ode to Time.",
    )))),
    likes: 432,
    comment_id: 1922667,
    time_since_post: Cow::Borrowed("6 days"),
};

load_save_roundtrip!(ProfileComment, PROFILE_COMMENT_DATA, PROFILE_COMMENT, "~", true);
save_load_roundtrip!(ProfileComment, ProfileComment, PROFILE_COMMENT);
