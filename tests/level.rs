use std::path::Path;

use dash_rs::model::level::Level;
use framework::load_test_units;

mod framework;

enum LevelTester {}

impl framework::Testable for LevelTester {
    type Target<'a> = Level<'a, ()>;

    fn canonicalize(level: &mut Self::Target<'_>) {
        if let Some(ref mut hunk) = level.description {
            hunk.process().unwrap();
        }
    }
}

#[test]
fn test_listed_level() {
    let units = load_test_units::<LevelTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("listed_level"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}

enum FullLevelTester {}

impl framework::Testable for FullLevelTester {
    type Target<'a> = Level<'a>;

    fn canonicalize(level: &mut Self::Target<'_>) {
        if let Some(ref mut hunk) = level.description {
            hunk.process().unwrap();
        }
        level.level_data.level_data.process().unwrap();
        level.level_data.password.process().unwrap();
    }
}

#[test]
fn test_full_level() {
    let units = load_test_units::<FullLevelTester>(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("artifacts").join("level"));

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        // Cannot do round trip testing for onw, as the level data handling in dash-rs is incomplete
        // (to put it nicely)
    }
}
