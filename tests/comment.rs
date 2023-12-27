use dash_rs::model::comment::{
    level::{CommentUser, LevelComment},
    profile::ProfileComment,
};
use framework::load_test_units;
use std::path::Path;

mod framework;

enum LevelCommentTester {}

impl framework::Testable for LevelCommentTester {
    type Target<'a> = LevelComment<'a>;

    fn canonicalize(target: &mut Self::Target<'_>) {
        if let Some(ref mut cnt) = target.content {
            cnt.process().unwrap();
        }
        if let Some(ref mut cnt) = target.special_color {
            cnt.process().unwrap();
        }
    }
}

#[test]
fn test_level_comment() {
    let units = load_test_units::<LevelCommentTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("level_comment"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}

enum CommentUserTester {}

impl framework::Testable for CommentUserTester {
    type Target<'a> = CommentUser<'a>;
}

#[test]
fn test_level_comment_user() {
    let units = load_test_units::<CommentUserTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("comment_user"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}

enum ProfileCommentTester {}

impl framework::Testable for ProfileCommentTester {
    type Target<'a> = ProfileComment<'a>;

    fn canonicalize(target: &mut Self::Target<'_>) {
        if let Some(ref mut cnt) = target.content {
            cnt.process().unwrap();
        }
    }
}

#[test]
fn test_profile_comment() {
    let units = load_test_units::<ProfileCommentTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("profile_comment"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}
