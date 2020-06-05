use crate::{
    model::level::object::{ids, speed::Speed, LevelObject, ObjectData},
    serde::HasRobtopFormat,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
pub struct InternalLevelObject {
    #[serde(rename = "1")]
    id: u16,

    #[serde(rename = "2")]
    x: f32,

    #[serde(rename = "3")]
    y: f32,

    #[serde(rename = "4", default)]
    flipped_x: bool,

    #[serde(rename = "5", default)]
    flipped_y: bool,

    #[serde(rename = "6", default)]
    rotation: f32,

    // ... other common fields

    // portal related fields
    #[serde(rename = "13", default)]
    checked: bool,
}

impl<'a> HasRobtopFormat<'a> for LevelObject {
    type Internal = InternalLevelObject;

    const DELIMITER: &'static str = ",";
    const MAP_LIKE: bool = true;

    fn as_internal(&'a self) -> Self::Internal {
        let mut internal = InternalLevelObject {
            id: self.id,
            x: self.x,
            y: self.y,
            flipped_x: self.flipped_x,
            flipped_y: self.flipped_y,
            rotation: self.rotation,
            ..InternalLevelObject::default()
        };

        match self.metadata {
            ObjectData::None | ObjectData::Unknown => {},
            ObjectData::SpeedPortal { checked, .. } => {
                internal.checked = checked;
            },
        };

        internal
    }

    fn from_internal(int: Self::Internal) -> Self {
        let metadata = match int.id {
            ids::SLOW_PORTAL =>
                ObjectData::SpeedPortal {
                    checked: int.checked,
                    speed: Speed::Slow,
                },
            ids::NORMAL_PORTAL =>
                ObjectData::SpeedPortal {
                    checked: int.checked,
                    speed: Speed::Normal,
                },
            ids::FAST_PORTAL =>
                ObjectData::SpeedPortal {
                    checked: int.checked,
                    speed: Speed::Fast,
                },
            ids::VERY_FAST_PORTAL =>
                ObjectData::SpeedPortal {
                    checked: int.checked,
                    speed: Speed::VeryFast,
                },
            _ => ObjectData::Unknown,
        };

        LevelObject {
            id: int.id,
            x: int.x,
            y: int.y,
            flipped_x: int.flipped_x,
            flipped_y: int.flipped_y,
            rotation: int.rotation,
            metadata,
        }
    }
}
