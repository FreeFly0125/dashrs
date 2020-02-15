use dash_rs::{
<<<<<<< HEAD
    de::thunk::{PercentDecoded, Thunk},
    model::{creator::Creator, song::NewgroundsSong},
    ser::indexed::IndexedSerializer,
};
use serde::Serialize;
=======
    model::{creator::Creator, song::NewgroundsSong},
    PercentDecoded, Thunk,
};
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b
use std::borrow::Cow;

const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

const CREO_DUNE_DATA: &str = "1~|~771277~|~2~|~Creo - \
<<<<<<< HEAD
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~10~|~https%3A%\
                              2F%2Faudio.ngfiles.com%2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1";
=======
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                              2F771277_Creo---Dune.mp3%3Ff1508708604~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1";
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b

/// Testing data for newgrounds song (de)serialization
///
/// This is the data provided by the Geometry Dash servers for the song "Dune" by Creo, except that
/// its fields have been reordered
const CREO_DUNE_DATA_ORDERED: &str = "1~|~771277~|~2~|~Creo - \
<<<<<<< HEAD
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%\
                              2F%2Faudio.ngfiles.com%2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604";

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%\
                              2F%2Faudio.ngfiles.com%2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";
=======
                                      Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                      03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                                      2F771277_Creo---Dune.mp3%3Ff1508708604";

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b

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
const CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

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
<<<<<<< HEAD
    let mut serializer = IndexedSerializer::new("~|~", true);

    assert!(CREO_DUNE.serialize(&mut serializer).is_ok());

    assert_eq!(serializer.finish(), CREO_DUNE_DATA_ORDERED);
=======
    let result = dash_rs::to_robtop_data(&CREO_DUNE);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), CREO_DUNE_DATA_ORDERED.as_bytes());
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b
}

#[test]
fn deserialize_song() {
<<<<<<< HEAD
    use dash_rs::model::song::from_str;

    let song = from_str(CREO_DUNE_DATA);
=======
    let song = dash_rs::from_robtop_str::<NewgroundsSong>(CREO_DUNE_DATA);
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b

    assert!(song.is_ok(), "{:?}", song.unwrap_err());

    let mut song = song.unwrap();

    assert!(song.link.process().is_ok());
    assert_eq!(song, CREO_DUNE);
}

#[test]
fn serialize_registered_creator() {
<<<<<<< HEAD
    let mut serializer = IndexedSerializer::new(":", false);

    assert!(CREATOR_REGISTERED.serialize(&mut serializer).is_ok());

    assert_eq!(serializer.finish(), CREATOR_REGISTERED_DATA);
=======
    let result = dash_rs::to_robtop_data(&CREATOR_REGISTERED);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), CREATOR_REGISTERED_DATA.as_bytes());
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b
}

#[test]
fn serialize_unregistered_creator() {
<<<<<<< HEAD
    let mut serializer = IndexedSerializer::new(":", false);

    assert!(CREATOR_UNREGISTERED.serialize(&mut serializer).is_ok());

    assert_eq!(serializer.finish(), CREATOR_UNREGISTERED_DATA);
=======
    let result = dash_rs::to_robtop_data(&CREATOR_UNREGISTERED);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), CREATOR_UNREGISTERED_DATA.as_bytes());
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b
}

#[test]
fn deserialize_registered_creator() {
<<<<<<< HEAD
    use dash_rs::model::creator::from_str;

    let creator = from_str(CREATOR_REGISTERED_DATA);
=======
    let creator = dash_rs::from_robtop_str::<Creator>(CREATOR_REGISTERED_DATA);
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b

    assert!(creator.is_ok(), "{:?}", creator.unwrap_err());
    assert_eq!(creator.unwrap(), CREATOR_REGISTERED);
}

#[test]
fn deserialize_unregistered_creator() {
<<<<<<< HEAD
    use dash_rs::model::creator::from_str;

    let creator = from_str(CREATOR_UNREGISTERED_DATA);
=======
    let creator = dash_rs::from_robtop_str::<Creator>(CREATOR_UNREGISTERED_DATA);
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b

    assert!(creator.is_ok(), "{:?}", creator.unwrap_err());
    assert_eq!(creator.unwrap(), CREATOR_UNREGISTERED);
}

#[test]
<<<<<<< HEAD
fn deserialize_too_many_fields(){
    use dash_rs::model::song::from_str;

    let song = from_str(CREO_DUNE_DATA_TOO_MANY_FIELDS);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());
}
=======
fn deserialize_too_many_fields() {
    let song = dash_rs::from_robtop_str::<NewgroundsSong>(CREO_DUNE_DATA_TOO_MANY_FIELDS);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());
}
>>>>>>> d6b3b4c68a85816020ef0eb0543533aaa641964b
