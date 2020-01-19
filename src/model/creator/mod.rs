use serde::{Deserialize, Serialize};

pub mod raw;

/// Struct representing a [`Level`]'s creator.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// The [`Creator`]'s unique user ID
    pub user_id: u64,

    /// The [`Creator`]'s name
    pub name: String,

    /// The [`Creator`]'s unique account ID
    pub account_id: Option<u64>,
}
