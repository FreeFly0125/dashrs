use dash_rs::{
    model::{
        creator::Creator,
        level::{DemonRating, Featured, Level, LevelData, LevelLength, LevelRating, Password},
        song::{MainSong, NewgroundsSong},
        GameVersion,
    },
    Thunk,
};
use std::borrow::Cow;

mod helper;

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

const TIME_PRESSURE: Level = Level {
    level_id: 897837,
    name: Cow::Borrowed("time pressure"),
    description: Some(Thunk::Processed(Cow::Borrowed("Fixed the bug at 91% 15/09/2020"))),
    version: 1,
    creator: 842519,
    difficulty: LevelRating::Demon(DemonRating::Easy),
    downloads: 7016929,
    main_song: Some(MainSong {
        main_song_id: 14,
        name: "Electrodynamix",
        artist: "DJ-Nate",
    }),
    gd_version: GameVersion::Version { minor: 1, major: 2 },
    likes: 277829,
    length: LevelLength::Long,
    stars: 10,
    featured: Featured::Featured(700),
    copy_of: Some(897837),
    two_player: false,
    custom_song: None,
    coin_amount: 0,
    coins_verified: false,
    stars_requested: None,
    is_epic: false,
    object_amount: Some(7092),
    index_46: Some(Cow::Borrowed("113")),
    index_47: Some(Cow::Borrowed("0")),
    level_data: LevelData {
        level_data: Thunk::Unprocessed(Cow::Borrowed("REMOVED")),
        password: Thunk::Processed(Password::PasswordCopy(3101)),
        time_since_upload: Cow::Borrowed("9 years"),
        time_since_update: Cow::Borrowed("3 years"),
        index_36: Cow::Borrowed(
            "0_167_67_0_0_0_0_207_0_0_89_88_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0",
        ),
        index_40: Cow::Borrowed("0"),
        index_52: Cow::Borrowed(""),
        index_53: Cow::Borrowed(""),
        index_57: Cow::Borrowed("0"),
    },
};

impl<S, U> helper::ThunkProcessor for Level<'_, LevelData<'_>, S, U> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut hunk) = self.description {
            hunk.process().unwrap();
        }
        self.level_data.level_data.process().unwrap();
        self.level_data.password.process().unwrap();
    }
}

impl<S, U> helper::ThunkProcessor for Level<'_, (), S, U> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut hunk) = self.description {
            hunk.process().unwrap();
        }
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

    let _ = helper::load_processed::<Level>(include_str!("data/11774780_dark_realm_gjdownload_response"));
}

#[test]
fn deserialize_level2() {
    init_log();

    let mut level = helper::load_processed::<Level>(include_str!("data/897837_time_pressure_gjdownload_response"));

    level.level_data.level_data = Thunk::Unprocessed(Cow::Borrowed("REMOVED"));

    dbg!(&level);

    assert_eq!(level, TIME_PRESSURE);
}

fn init_log() {
    if let Err(err) = env_logger::builder().is_test(true).try_init() {
        // nothing to make the tests fail over
        eprintln!("Error setting up env_logger: {:?}", err)
    }
}
