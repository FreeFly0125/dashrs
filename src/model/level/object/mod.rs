use crate::model::level::object::speed::Speed;
use serde::{Deserialize, Serialize};

pub mod ids;
mod internal;
pub mod speed;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct LevelObject {
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub flipped_x: bool,
    pub flipped_y: bool,
    pub rotation: f32,
    // ... other fields they all have ...
    pub metadata: ObjectData,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ObjectData {
    None,
    Unknown,
    SpeedPortal { checked: bool, speed: Speed },
}
