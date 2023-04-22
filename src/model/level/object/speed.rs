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
