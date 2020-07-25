// TODO: Speed portals and stuff
use serde::{Deserialize, Serialize};

/// Enum modelling the different speeds a player can have during gameplay
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Speed {
    Slow,
    Normal,
    Medium,
    Fast,
    VeryFast,
    Unknown(u8),
}

impl Default for Speed {
    fn default() -> Speed {
        Speed::Normal
    }
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
