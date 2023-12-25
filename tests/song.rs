use dash_rs::{model::song::NewgroundsSong, Thunk};
use std::borrow::Cow;

#[macro_use]
mod helper;

const CREO_DUNE_DATA: &str = "1~|~771277~|~2~|~Creo - \
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                              2F771277_Creo---Dune.mp3%3Ff1508708604~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1";

const CREO_DUNE: NewgroundsSong<'static> = NewgroundsSong {
    song_id: 771277,
    name: Cow::Borrowed("Creo - Dune"),
    index_3: 50531,
    artist: Cow::Borrowed("CreoMusic"),
    filesize: 8.03,
    index_6: None,
    index_7: Some(Cow::Borrowed("UCsCWA3Y3JppL6feQiMRgm6Q")),
    index_8: Cow::Borrowed("1"),
    link: Thunk::Processed(Cow::Borrowed("https://audio.ngfiles.com/771000/771277_Creo---Dune.mp3?f1508708604")),
};

impl<'a> helper::ThunkProcessor for NewgroundsSong<'a> {
    fn process_all_thunks(&mut self) {
        assert!(self.link.process().is_ok());
    }
}

save_load_roundtrip!(NewgroundsSong, CREO_DUNE);
load_save_roundtrip!(NewgroundsSong, CREO_DUNE_DATA, CREO_DUNE, "~|~", true);
