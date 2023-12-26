use dash_rs::model::user::profile::Profile;
use framework::load_test_units;
use std::path::Path;

mod framework;

enum ProfileTester {}

impl framework::Testable for ProfileTester {
    type Target<'a> = Profile<'a>;
}

#[test]
fn test_profile() {
    let units = load_test_units::<ProfileTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("profile"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}
