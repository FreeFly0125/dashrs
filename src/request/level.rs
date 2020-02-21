use serde::Serialize;
use serde::Deserialize;
use crate::request::BaseRequest;

/// Struct modelled after a request to `downloadGJLevel22.php`.
///
/// In the Geometry Dash API, this endpoint is used to download a level from
/// the servers and retrieve some additional information that isn't provided
/// with the response to a [`LevelsRequest`]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct LevelRequest<'a> {
    /// The base request data
    #[serde(borrow)]
    pub base: BaseRequest<'a>,

    /// The ID of the level to download
    ///
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// Some weird field the Geometry Dash Client sends along
    ///
    /// ## GD Internals:
    /// This value needs to be converted to an integer for the boomlings API
    pub inc: bool,

    /// Some weird field the Geometry Dash Client sends along
    ///
    /// ## GD Internals:
    /// This field is called `extras` in the boomlings API and needs to be
    /// converted to an integer
    pub extra: bool,
}