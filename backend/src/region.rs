use serde::{Deserialize, Serialize};
use std::convert::From;

pub mod fence {
    tonic::include_proto!("fence");
}

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

/// Region from the gRPC Service
impl From<Region> for fence::Region {
    fn from(region: Region) -> Self {
        Self {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
            id: region.id,
        }
    }
}

// impl From<fence::Region> for Region {
//     fn from(region: fence::Region) -> Self {
//         Self {
//             x: region.x,
//             y: region.y,
//             width: region.width,
//             height: region.height,
//             id: region.id,
//         }
//     }
// }
