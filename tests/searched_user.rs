use dash_rs::model::user::searched::SearchedUser;
use framework::load_test_units;
use std::path::Path;

mod framework;

enum SearchedUserTester {}

impl framework::Testable for SearchedUserTester {
    type Target<'a> = SearchedUser<'a>;
}

#[test]
fn test_searched_user() {
    let units = load_test_units::<SearchedUserTester>(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("artifacts")
            .join("searched_user"),
    );

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}
