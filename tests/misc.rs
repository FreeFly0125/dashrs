use dash_rs::{
    model::{creator::Creator, song::NewgroundsSong},
    GJFormat,
};

mod framework;

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

#[test]
fn deserialize_too_many_fields() {
    // Superfluous fields should just be ignored
    NewgroundsSong::from_gj_str(CREO_DUNE_DATA_TOO_MANY_FIELDS).unwrap();
    Creator::from_gj_str(CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS).unwrap();
}
