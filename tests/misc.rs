use dash_rs::{
    model::{
        creator::Creator,
        level::{DemonRating, Featured, Level, LevelData, LevelLength, LevelRating, Password},
        song::{MainSong, NewgroundsSong},
        GameVersion,
    },
    Base64Decoded, Thunk,
};
use std::borrow::Cow;

mod helper;

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

const TIME_PRESSURE: Level<'static, Option<u64>, u64> = Level {
    level_id: 897837,
    name: Cow::Borrowed("time pressure"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "please rate and like  8-9 stars mabye?",
    )))),
    version: 2,
    creator: 842519,
    difficulty: LevelRating::Demon(DemonRating::Easy),
    downloads: 3189574,
    main_song: Some(MainSong {
        main_song_id: 14,
        name: "Electrodynamix",
        artist: "DJ-Nate",
    }),
    gd_version: GameVersion::Unknown,
    likes: 198542,
    length: LevelLength::Long,
    stars: 10,
    featured: Featured::Featured(700),
    copy_of: None,
    index_31: Some(Cow::Borrowed("0")),
    custom_song: None,
    coin_amount: 0,
    coins_verified: false,
    stars_requested: None,
    is_epic: false,
    index_43: Cow::Borrowed("3"),
    object_amount: None,
    index_46: None,
    index_47: None,
    level_data: Some(LevelData {
        level_data: Thunk::Unprocessed("REMOVED"),
        password: Password::PasswordCopy(3101),
        time_since_upload: Cow::Borrowed("5 years"),
        time_since_update: Cow::Borrowed("5 years"),
        index_36: None,
    }),
};

impl<S, U> helper::ThunkProcessor for Level<'_, S, U> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut hunk) = self.description {
            assert!(hunk.process().is_ok())
        }
        assert!(self.level_data.is_some());
        let data = self.level_data.as_mut().unwrap();
        let objects = data.level_data.process();
        assert!(objects.is_ok(), "{:?}", objects.unwrap_err());
    }
}

#[test]
fn deserialize_too_many_fields() {
    init_log();

    helper::load::<NewgroundsSong>(CREO_DUNE_DATA_TOO_MANY_FIELDS);
    helper::load::<Creator>(CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS);
}

#[test]
fn deserialize_level() {
    init_log();

    let mut level = helper::load_processed::<Level<Option<u64>, u64>>(include_str!("data/11774780_dark_realm_gjdownload_response"));
}

#[test]
fn deserialize_level2() {
    init_log();

    let mut level = helper::load_processed::<Level<Option<u64>, u64>>(include_str!("data/897837_time_pressure_gjdownload_response"));

    level.level_data.as_mut().unwrap().level_data = Thunk::Unprocessed("REMOVED");

    assert_eq!(level, TIME_PRESSURE);
}

fn init_log() {
    if let Err(err) = env_logger::builder().is_test(true).try_init() {
        // nothing to make the tests fail over
        eprintln!("Error setting up env_logger: {:?}", err)
    }
}
