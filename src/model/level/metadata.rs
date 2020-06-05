use crate::model::level::object::speed::Speed;

#[derive(Debug, PartialEq, Clone, Default, Copy)]
pub struct LevelMetadata {
    pub starting_speed: Speed,
    pub song_offset: f64,
    pub song_fade_in: bool,
    pub song_fade_out: bool,
    pub dual_start: bool,
    pub two_player_controls: bool,
    pub start_gravity_inverted: bool,
    // ... other fields in the metadata section ...
}

mod internal {
    use serde::{Serialize, Deserialize};
    use crate::serde::HasRobtopFormat;
    use crate::model::level::metadata::LevelMetadata;
    use crate::model::level::object::speed::Speed;

    impl<'a> HasRobtopFormat<'a> for LevelMetadata {
        type Internal = InternalLevelMetadata;

        const DELIMITER: &'static str = ",";
        const MAP_LIKE: bool = true;

        fn as_internal(&'a self) -> Self::Internal {
            InternalLevelMetadata {
                starting_speed: match self.starting_speed {
                    Speed::Slow => 0,
                    Speed::Normal => 1,
                    Speed::Medium => 2,
                    Speed::Fast => 3,
                    Speed::VeryFast => 4,
                    Speed::Unknown(unknown) => unknown,
                },
                song_offset: self.song_offset,
                song_fade_in: self.song_fade_in,
                song_fade_out: self.song_fade_out,
                dual_start: self.dual_start,
                two_player_controls: self.two_player_controls,
                start_gravity_inverted: self.start_gravity_inverted
            }
        }

        fn from_internal(int: Self::Internal) -> Self {
            LevelMetadata {
                starting_speed: match int.starting_speed {
                    0 => Speed::Slow,
                    1 => Speed::Normal,
                    2 => Speed::Medium,
                    3 => Speed::Fast,
                    4 => Speed::VeryFast,
                    unknown => Speed::Unknown(unknown)
                },
                song_offset: int.song_offset,
                song_fade_in: int.song_fade_in,
                song_fade_out: int.song_fade_out,
                dual_start: int.dual_start,
                two_player_controls: int.two_player_controls,
                start_gravity_inverted: int.start_gravity_inverted
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct InternalLevelMetadata {
        #[serde(rename = "kA4")]
        starting_speed: u8,

        #[serde(rename = "kA13")]
        song_offset: f64,

        #[serde(rename = "kA15")]
        song_fade_in: bool,

        #[serde(rename = "kA16")]
        song_fade_out: bool,

        #[serde(rename = "kA8")]
        dual_start:bool,

        #[serde(rename = "kA10")]
        two_player_controls:bool,

        #[serde(rename = "kA11")]
        start_gravity_inverted: bool

        //.. other fields
    }
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
