use crate::{
    model::level::object::{ids, speed::Speed, LevelObject, ObjectData},
    Dash, GJFormat,
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

impl<'de> Dash<'de> for LevelObject {
    fn dash_deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = InternalLevelObject::deserialize(deserializer)?;

        let metadata = match internal.id {
            ids::SLOW_PORTAL => ObjectData::SpeedPortal {
                checked: internal.checked,
                speed: Speed::Slow,
            },
            ids::NORMAL_PORTAL => ObjectData::SpeedPortal {
                checked: internal.checked,
                speed: Speed::Normal,
            },
            ids::FAST_PORTAL => ObjectData::SpeedPortal {
                checked: internal.checked,
                speed: Speed::Fast,
            },
            ids::VERY_FAST_PORTAL => ObjectData::SpeedPortal {
                checked: internal.checked,
                speed: Speed::VeryFast,
            },
            _ => ObjectData::Unknown,
        };

        Ok(LevelObject {
            id: internal.id,
            x: internal.x,
            y: internal.y,
            flipped_x: internal.flipped_x,
            flipped_y: internal.flipped_y,
            rotation: internal.rotation,
            metadata,
        })
    }

    fn dash_serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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

        internal.serialize(serializer)
    }
}

impl<'de> GJFormat<'de> for LevelObject {
    const DELIMITER: &'static str = ",";
    const MAP_LIKE: bool = true;
}
