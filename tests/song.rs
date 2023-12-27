use dash_rs::model::song::NewgroundsSong;
use framework::load_test_units;
use std::path::Path;

mod framework;

enum NewgroundsSongTester {}

impl framework::Testable for NewgroundsSongTester {
    type Target<'a> = NewgroundsSong<'a>;

    fn canonicalize(target: &mut Self::Target<'_>) {
        target.link.process().unwrap();
    }
}

#[test]
fn test_newgrounds_song() {
    let units = load_test_units::<NewgroundsSongTester>(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("artifacts").join("song"));

    for (path, unit) in units {
        println!("Testing case {:?}", path);

        unit.test_consistency();
        unit.test_load_save_roundtrip();
        unit.test_save_load_roundtrip();
    }
}
