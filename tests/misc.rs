use dash_rs::model::{creator::Creator, level::Level, song::NewgroundsSong};
use framework::load_test_units;
use std::path::Path;

mod framework;
mod helper;

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

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
        // Cannot do round trip testing for onw, as the level data handling in dash-rs is incomplete (to put it nicely)
    }
}

#[test]
fn deserialize_too_many_fields() {
    init_log();

    helper::load::<NewgroundsSong>(CREO_DUNE_DATA_TOO_MANY_FIELDS);
    helper::load::<Creator>(CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS);
}

fn init_log() {
    if let Err(err) = env_logger::builder().is_test(true).try_init() {
        // nothing to make the tests fail over
        eprintln!("Error setting up env_logger: {:?}", err)
    }
}
