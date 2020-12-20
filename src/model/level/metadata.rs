use crate::model::level::object::speed::Speed;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Copy, Serialize, Deserialize)]
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
    use crate::{
        model::level::{metadata::LevelMetadata, object::speed::Speed},
        serde::{HasRobtopFormat, IndexedDeserializer, IndexedSerializer},
        DeError, SerError,
    };
    use serde::{Deserialize, Serialize};
    use std::io::Write;

    impl<'a> HasRobtopFormat<'a> for LevelMetadata {
        fn from_robtop_str(input: &'a str) -> Result<Self, DeError> {
            let int = InternalLevelMetadata::deserialize(&mut IndexedDeserializer::new(input, ",", true))?;

            Ok(LevelMetadata {
                starting_speed: match int.starting_speed {
                    0 => Speed::Slow,
                    1 => Speed::Normal,
                    2 => Speed::Medium,
                    3 => Speed::Fast,
                    4 => Speed::VeryFast,
                    unknown => Speed::Unknown(unknown),
                },
                song_offset: int.song_offset,
                song_fade_in: int.song_fade_in,
                song_fade_out: int.song_fade_out,
                dual_start: int.dual_start,
                two_player_controls: int.two_player_controls,
                start_gravity_inverted: int.start_gravity_inverted,
            })
        }

        fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
            let internal = InternalLevelMetadata {
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
                start_gravity_inverted: self.start_gravity_inverted,
            };

            internal.serialize(&mut IndexedSerializer::new(",", writer, true))
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct InternalLevelMetadata {
        #[serde(rename = "kA4", default = "one")]
        starting_speed: u8,

        #[serde(rename = "kA13", default)]
        song_offset: f64,

        #[serde(rename = "kA15", default)]
        song_fade_in: bool,

        #[serde(rename = "kA16", default)]
        song_fade_out: bool,

        #[serde(rename = "kA8", default)]
        dual_start: bool,

        #[serde(rename = "kA10", default)]
        two_player_controls: bool,

        #[serde(rename = "kA11", default)]
        start_gravity_inverted: bool, //.. other fields
    }

    fn one() -> u8 {
        1
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
