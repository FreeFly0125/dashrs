use std::borrow::Cow;

use dash_rs::{
    model::{
        level::{DemonRating, Featured, Level, LevelLength, LevelRating},
        song::MainSong,
        GameVersion,
    },
    Base64Decoded, Thunk,
};

#[macro_use]
mod helper;

// A 1.9 level
const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

// A 1.3 (?) level
const DEMON_WORLD_DATA: &str = "1:72540:2:demon \
                                world:5:7:6:37573:8:10:9:30:10:272374:12:9:13:7:14:-3628:17:1:43:0:25::18:10:19:0:42:0:45:0:3:\
                                aGFwcHkgbmV3IHllYXIhIQ==:15:3:30:0:31:0:37:0:38:0:39:0:46:1:47:2:35:0";

// A 2.1 level
const FANTASY_DATA: &str = "1:63355989:2:Fantasy:5:2:6:15557115:8:10:9:40:10:9352:12:0:13:21:14:912:17::43:5:25::18:7:19:24978:42:0:45:\
                            37866:3:Q29sbGFiIHdpdGggQnJpbmRpa3osIHRoYW5rIHlvdSBmb3IgdGhpcyBsZXZlbCB1d3UsIEVOSk9ZISEg:15:3:30:63309629:31:\
                            0:37:2:38:1:39:7:46:1:47:2:35:771517";

// A 2.0 level in two-player mode (also an extreme demon)
const DUELO_MAETSTRO_DATA: &str = "1:23298409:2:Duelo Maestro:5:8:6:1295392:8:10:9:40:10:3302831:12:0:13:21:14:268067:17:1:43:5:25::18:10:19:0:42:0:45:45133:3:RWwgZHVlbG8gZGUgdHVzIGRvcyBtYW5vcyBvIGRlIGRvcyB2ZXJkYWRlcm9zIG1hZXN0cm9zLiBBIHZlY2VzIHB1ZWRlcyBtb3JpciBpbmV4cGxpY2FibGVtZW50ZSBlbiBsYSBwcmltZXJhIGJvbGEsIHBvcmZhIHJlaW5pY2llbiBlbCBuaXZlbC4=:15:4:30:0:31:1:37:2:38:1:39:10:46:1:47:2:35:645631";

const DARK_REALM: Level<()> = Level {
    level_id: 11774780,
    name: Cow::Borrowed("Dark Realm"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "My best level yet. Video on my YouTube. Have fun in this fast-paced DEMON >:) v2 Fixed some things",
    )))),
    version: 2,
    creator: 2073761,
    difficulty: LevelRating::Demon(DemonRating::Hard),
    downloads: 90786,
    main_song: None,
    gd_version: GameVersion::Version { minor: 0, major: 2 },
    likes: 10974,
    length: LevelLength::Long,
    stars: 10,
    featured: Featured::Featured(11994),
    copy_of: None,
    two_player: false,
    custom_song: Some(444085),
    coin_amount: 3,
    coins_verified: true,
    stars_requested: Some(10),
    is_epic: false,
    object_amount: None,
    index_46: Some(Cow::Borrowed("1")),
    index_47: Some(Cow::Borrowed("2")),
    level_data: (),
};

const DEMON_WORLD: Level<()> = Level {
    level_id: 72540,
    name: Cow::Borrowed("demon world"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed("happy new year!!")))),
    version: 7,
    creator: 37573,
    difficulty: LevelRating::Demon(DemonRating::Hard),
    downloads: 272374,
    main_song: Some(MainSong {
        main_song_id: 9,
        name: "xStep",
        artist: "DJVI",
    }),
    gd_version: GameVersion::Version { minor: 7, major: 0 },
    likes: -3628,
    length: LevelLength::Long,
    stars: 10,
    featured: Featured::NotFeatured,
    copy_of: None,
    two_player: false,
    custom_song: None,
    coin_amount: 0,
    coins_verified: false,
    stars_requested: None,
    is_epic: false,
    object_amount: None,
    index_46: Some(Cow::Borrowed("1")),
    index_47: Some(Cow::Borrowed("2")),
    level_data: (),
};

const FANTASY: Level<()> = Level {
    level_id: 63355989,
    name: Cow::Borrowed("Fantasy"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "Collab with Brindikz, thank you for this level uwu, ENJOY!! ",
    )))),
    version: 2,
    creator: 15557115,
    difficulty: LevelRating::Harder,
    downloads: 9352,
    main_song: None,
    gd_version: GameVersion::Version { minor: 1, major: 2 },
    likes: 912,
    length: LevelLength::Long,
    stars: 7,
    featured: Featured::Featured(24978),
    copy_of: Some(63309629),
    two_player: false,
    custom_song: Some(771517),
    coin_amount: 2,
    coins_verified: true,
    stars_requested: Some(7),
    is_epic: false,
    object_amount: Some(37866),
    index_46: Some(Cow::Borrowed("1")),
    index_47: Some(Cow::Borrowed("2")),
    level_data: (),
};

const DUELO_MAESTRO: Level<()> = Level {
    level_id: 23298409,
    name: Cow::Borrowed("Duelo Maestro"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "El duelo de tus dos manos o de dos verdaderos maestros. A veces puedes morir inexplicablemente en la primera bola, porfa \
         reinicien el nivel.",
    )))),
    version: 8,
    creator: 1295392,
    difficulty: LevelRating::Demon(DemonRating::Insane),
    downloads: 3302831,
    main_song: None,
    gd_version: GameVersion::Version { minor: 1, major: 2 },
    likes: 268067,
    length: LevelLength::ExtraLong,
    stars: 10,
    featured: Featured::NotFeatured,
    copy_of: None,
    two_player: true,
    custom_song: Some(645631),
    coin_amount: 2,
    coins_verified: true,
    stars_requested: Some(10),
    is_epic: false,
    object_amount: Some(45133),
    index_46: Some(Cow::Borrowed("1")),
    index_47: Some(Cow::Borrowed("2")),
    level_data: (),
};

impl<S, U> helper::ThunkProcessor for Level<'_, (), S, U> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut hunk) = self.description {
            assert!(hunk.process().is_ok())
        }
    }
}

save_load_roundtrip!(save_load_roundtrip_dark_realm, Level<()>, DARK_REALM);
load_save_roundtrip!(load_save_roundtrip_dark_realm, Level<()>, DARK_REALM_DATA, DARK_REALM, ":", true);

save_load_roundtrip!(save_load_roundtrip_demon_world, Level<()>, DEMON_WORLD);
load_save_roundtrip!(load_save_roundtrip_demon_world, Level<()>, DEMON_WORLD_DATA, DEMON_WORLD, ":", true);

save_load_roundtrip!(save_load_roundtrip_fantasy, Level<()>, FANTASY);
load_save_roundtrip!(load_save_roundtrip_fantasy, Level<()>, FANTASY_DATA, FANTASY, ":", true);

save_load_roundtrip!(save_load_roundtrip_duelo_maestro, Level<()>, DUELO_MAESTRO);
load_save_roundtrip!(
    load_save_roundtrip_duelo_maestro,
    Level<()>,
    DUELO_MAETSTRO_DATA,
    DUELO_MAESTRO,
    ":",
    true
);
