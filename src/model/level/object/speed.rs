// TODO: Speed portals and stuff
use serde::{Deserialize, Serialize};

/// Enum modelling the different speeds a player can have during gameplay
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Speed {
    Slow,
    #[default]
    Normal,
    Medium,
    Fast,
    VeryFast,
    Unknown(u8),
}

/// Converts the speed to the game-internal "pixel  / second" value represented by some [`Speed`]
/// variant
impl From<Speed> for f32 {
    fn from(speed: Speed) -> f32 {
        match speed {
            Speed::Unknown(_) => 0.0,
            Speed::Slow => 251.16,
            Speed::Normal => 311.58,
            Speed::Medium => 387.42,
            Speed::Fast => 468.0,
            Speed::VeryFast => 576.0,
        }
    }
}

impl From<u8> for Speed {
    fn from(value: u8) -> Self {
        match value {
            0 => Speed::Slow,
            1 => Speed::Normal,
            2 => Speed::Medium,
            3 => Speed::Fast,
            4 => Speed::VeryFast,
            unknown => Speed::Unknown(unknown),
        }
    }
}

impl From<Speed> for u8 {
    fn from(speed: Speed) -> Self {
        match speed {
            Speed::Slow => 0,
            Speed::Normal => 1,
            Speed::Medium => 2,
            Speed::Fast => 3,
            Speed::VeryFast => 4,
            Speed::Unknown(unknown) => unknown,
        }
    }
}

crate::into_conversion!(Speed, u8);
