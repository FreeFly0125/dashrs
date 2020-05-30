use dash_rs::{
    model::{creator::Creator, level::Level, song::NewgroundsSong},
    PercentDecoded, Thunk,
};
use std::borrow::Cow;
use dash_rs::Base64Decoded;
use dash_rs::model::level::{LevelRating, DemonRating, LevelLength, LevelData, Password};
use dash_rs::model::song::MainSong;
use dash_rs::model::GameVersion;
use dash_rs::model::level::Featured::Featured;

const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

const CREO_DUNE_DATA: &str = "1~|~771277~|~2~|~Creo - \
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                              2F771277_Creo---Dune.mp3%3Ff1508708604~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1";

/// Testing data for newgrounds song (de)serialization
///
/// This is the data provided by the Geometry Dash servers for the song "Dune" by Creo, except that
/// its fields have been reordered
const CREO_DUNE_DATA_ORDERED: &str = "1~|~771277~|~2~|~Creo - \
                                      Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                      03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                                      2F771277_Creo---Dune.mp3%3Ff1508708604";

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const CREO_DUNE: NewgroundsSong<'static> = NewgroundsSong {
    song_id: 771277,
    name: Cow::Borrowed("Creo - Dune"),
    index_3: 50531,
    artist: Cow::Borrowed("CreoMusic"),
    filesize: 8.03,
    index_6: None,
    index_7: Some(Cow::Borrowed("UCsCWA3Y3JppL6feQiMRgm6Q")),
    index_8: Cow::Borrowed("1"),
    link: Thunk::Processed(PercentDecoded(Cow::Borrowed(
        "https://audio.ngfiles.com/771000/771277_Creo---Dune.mp3?f1508708604",
    ))),
};

const CREATOR_REGISTERED_DATA: &str = "4170784:Serponge:119741";
const _CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

const CREATOR_REGISTERED: Creator = Creator {
    user_id: 4170784,
    name: Cow::Borrowed("Serponge"),
    account_id: Some(119741),
};

const CREATOR_UNREGISTERED_DATA: &str = "4170784:Serponge:0";

const CREATOR_UNREGISTERED: Creator = Creator {
    user_id: 4170784,
    name: Cow::Borrowed("Serponge"),
    account_id: None,
};

const TIME_PRESSURE: Level<Option<u64>,u64> = Level {
    level_id: 897837,
    name: Cow::Borrowed("time pressure"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed("please rate and like  8-9 stars mabye?")))),
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
    featured: Featured(700),
    copy_of: None,
    index_31: Some(Cow::Borrowed("0")),
    custom_song: None,
    coin_amount: 0,
    coins_verified: false,
    stars_requested: None,
    index_40: None,
    is_epic: false,
    index_43: Cow::Borrowed("3"),
    object_amount: None,
    index_46: None,
    index_47: None,
    level_data: Some(LevelData {
        level_data: Cow::Borrowed("REMOVED"),
        password: Password::PasswordCopy(3101),
        time_since_upload: Cow::Borrowed("5 years"),
        time_since_update: Cow::Borrowed("5 years"),
        index_36: None,
    }),
};

#[test]
fn serialize_song() {
    env_logger::builder().is_test(true).try_init();

    let mut buf: Vec<u8> = Vec::new();
    let result = dash_rs::write_robtop_data(&CREO_DUNE, &mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, CREO_DUNE_DATA_ORDERED.as_bytes());
}

#[test]
fn deserialize_song() {
    env_logger::builder().is_test(true).try_init();

    let song = dash_rs::from_robtop_str::<NewgroundsSong>(CREO_DUNE_DATA);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());

    let mut song = song.unwrap();

    assert!(song.link.process().is_ok());
    assert_eq!(song, CREO_DUNE);
}

#[test]
fn serialize_registered_creator() {
    env_logger::builder().is_test(true).try_init();

    let mut buf: Vec<u8> = Vec::new();
    let result = dash_rs::write_robtop_data(&CREATOR_REGISTERED, &mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, CREATOR_REGISTERED_DATA.as_bytes());
}

#[test]
fn serialize_unregistered_creator() {
    env_logger::builder().is_test(true).try_init();

    let mut buf: Vec<u8> = Vec::new();
    let result = dash_rs::write_robtop_data(&CREATOR_UNREGISTERED, &mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, CREATOR_UNREGISTERED_DATA.as_bytes());
}

#[test]
fn deserialize_registered_creator() {
    env_logger::builder().is_test(true).try_init();

    let creator = dash_rs::from_robtop_str::<Creator>(CREATOR_REGISTERED_DATA);

    assert!(creator.is_ok(), "{:?}", creator.unwrap_err());
    assert_eq!(creator.unwrap(), CREATOR_REGISTERED);
}

#[test]
fn deserialize_unregistered_creator() {
    env_logger::builder().is_test(true).try_init();

    let creator = dash_rs::from_robtop_str::<Creator>(CREATOR_UNREGISTERED_DATA);

    assert!(creator.is_ok(), "{:?}", creator.unwrap_err());
    assert_eq!(creator.unwrap(), CREATOR_UNREGISTERED);
}

#[test]
fn deserialize_too_many_fields() {
    env_logger::builder().is_test(true).try_init();

    let song = dash_rs::from_robtop_str::<NewgroundsSong>(CREO_DUNE_DATA_TOO_MANY_FIELDS);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());
}

#[test]
fn deserialize_partial_level() {
    env_logger::builder().is_test(true).try_init();

    let level = dash_rs::from_robtop_str::<Level<_, _>>(DARK_REALM_DATA);

    assert!(level.is_ok(), "{:?}", level.unwrap_err());

    let mut level = level.unwrap();

    assert!(level.description.as_mut().unwrap().process().is_ok());
}

#[test]
fn deserialize_level() {
    env_logger::builder().is_test(true).try_init();

    let level = dash_rs::from_robtop_str::<Level<_, _>>(include_str!("data/11774780_dark_realm_gjdownload_response"));

    let mut level = level.unwrap();

    assert!(level.description.as_mut().unwrap().process().is_ok());
    assert!(level.level_data.is_some());
}

#[test]
fn deserialize_level2() {
    env_logger::builder().is_test(true).try_init();

    let level = dash_rs::from_robtop_str::<Level<_, _>>(include_str!("data/897837_time_pressure_gjdownload_response"));

    let mut level = level.unwrap();

    assert!(level.description.as_mut().unwrap().process().is_ok());
    assert!(level.level_data.is_some());

    level.level_data.as_mut().unwrap().level_data = Cow::Borrowed("REMOVED");

    assert_eq!(level, TIME_PRESSURE);
}
