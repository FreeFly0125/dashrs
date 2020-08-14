use std::borrow::Cow;

use dash_rs::{Base64Decoded, Thunk};
use dash_rs::model::GameVersion;
use dash_rs::model::level::{DemonRating, Level, LevelLength, LevelRating};
use dash_rs::model::level::Featured::Featured;

#[macro_use]
mod helper;

const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

const DARK_REALM: Level<Option<u64>, u64> = Level {
    level_id: 11774780,
    name: Cow::Borrowed("Dark Realm"),
    description: Some(
        Thunk::Processed(
            Base64Decoded(
                Cow::Borrowed("My best level yet. Video on my YouTube. Have fun in this fast-paced DEMON >:) v2 Fixed some things"),
            ),
        ),
    ),
    version: 2,
    creator: 2073761,
    difficulty: LevelRating::Demon(
        DemonRating::Hard,
    ),
    downloads: 90786,
    main_song: None,
    gd_version: GameVersion::Version {
        minor: 0,
        major: 2,
    },
    likes: 10974,
    length: LevelLength::Long,
    stars: 10,
    featured: Featured(
        11994,
    ),
    copy_of: None,
    index_31: Some(
        Cow::Borrowed("0"),
    ),
    custom_song: Some(
        444085,
    ),
    coin_amount: 3,
    coins_verified: true,
    stars_requested: Some(
        10,
    ),
    index_40: None,
    is_epic: false,
    index_43: Cow::Borrowed("0"),
    object_amount: None,
    index_46: Some(
        Cow::Borrowed("1"),
    ),
    index_47: Some(
        Cow::Borrowed("2"),
    ),
    level_data: None,
};

impl<S, U> helper::ThunkProcessor for Level<'_, S, U> {
    fn process_all_thunks(&mut self) {
        if let Some(ref mut hunk) = self.description {
            assert!(hunk.process().is_ok())
        }
    }
}

save_load_roundtrip!(Level<Option<u64>, u64>, DARK_REALM);
load_save_roundtrip!(Level<Option<u64>, u64>, DARK_REALM_DATA, DARK_REALM, ":", true);