use serde::{Deserialize, Serialize};

use crate::grpc::fence;

/// Region
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    /// The Top Position of the Region
    pub x: i32,
    /// The Left Position of the Region
    pub y: i32,
    /// The Width of the Region
    pub width: i32,
    /// The Height of the Region
    pub height: i32,
    /// The ID of the Region
    pub id: String,
}

impl From<fence::Region> for crate::region::Region {
    fn from(region: fence::Region) -> Self {
        Self {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
            id: region.id,
        }
    }
}

impl From<&crate::region::Region> for fence::Region {
    fn from(region: &crate::region::Region) -> Self {
        Self {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
            id: region.id.to_owned(),
        }
    }
}
