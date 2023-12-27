use dash_rs::model::{creator::Creator, song::NewgroundsSong};

mod framework;
mod helper;

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

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
