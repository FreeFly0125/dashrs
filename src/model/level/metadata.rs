use crate::{model::level::object::speed::Speed, GJFormat};
use dash_rs_derive::Dash;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Copy, Serialize, Deserialize, Dash)]
pub struct LevelMetadata {
    #[dash(index = "kA4")]
    #[dash(default = "one")]
    pub starting_speed: Speed,

    #[dash(index = "kA13")]
    #[dash(default)]
    pub song_offset: f64,

    #[dash(index = "kA15")]
    #[dash(default)]
    pub song_fade_in: bool,

    #[dash(index = "kA16")]
    #[dash(default)]
    pub song_fade_out: bool,

    #[dash(index = "kA8")]
    #[dash(default)]
    pub dual_start: bool,

    #[dash(index = "kA10")]
    #[dash(default)]
    pub two_player_controls: bool,

    #[dash(index = "kA11")]
    #[dash(default)]
    pub start_gravity_inverted: bool,
    // ... other fields in the metadata section ...
}

impl<'de> GJFormat<'de> for LevelMetadata {
    const DELIMITER: &'static str = ",";
    const MAP_LIKE: bool = true;
}

fn one() -> u8 {
    1
}

// starting_speed(index = kA4),
// song_offset(index = kA13),
// fade_in(index = kA15),
// fade_out(index = kA16),
// song guidelines: kA16
// background texture index: kA6
// ground texture index: kA7
// ground line index: kA17
// font: kA18
// color page (???): kS39
// starting game mode: kA2
// starting size: kA3
// dual_start(index = kA8),
// level/start pos (???): kA9
// two_player_controls(index = kA10),
// start_gravity_inverted(index = kA11, optional),
