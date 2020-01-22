use dash_rs::{
    de::thunk::{PercentDecoded, Thunk},
    model::{creator::Creator, song::NewgroundsSong},
    ser::indexed::IndexedSerializer,
};
use serde::Serialize;
use std::borrow::Cow;

const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

/// Testing data for newgrounds song (de)serialization
///
/// This is the data provided by the Geometry Dash servers for the song "Dune" by Creo, except that
/// its fields have been reordered
const CREO_DUNE_DATA: &str = "1~|~771277~|~2~|~Creo - \
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%\
                              2F%2Faudio.ngfiles.com%2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604";

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

#[test]
fn serialize_song() {
    let mut serializer = IndexedSerializer::new("~|~", true);

    assert!(CREO_DUNE.serialize(&mut serializer).is_ok());

    assert_eq!(serializer.finish(), CREO_DUNE_DATA);
}

#[test]
fn deserialize_song() {
    use dash_rs::model::song::from_str;

    let song = from_str(CREO_DUNE_DATA);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());

    let mut song = song.unwrap();

    assert!(song.link.process().is_ok());
    assert_eq!(song, CREO_DUNE);
}
