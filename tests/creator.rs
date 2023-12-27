use dash_rs::model::creator::Creator;
use framework::load_test_units;
use std::path::Path;

mod framework;

enum CreatorTester {}

impl framework::Testable for CreatorTester {
    type Target<'a> = Creator<'a>;
}

#[test]
fn test_creator() {
    let units = load_test_units::<CreatorTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("creator"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}
