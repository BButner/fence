use serde::{Deserialize, Serialize};

/// Region
#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    /// The Top Position of the Region
    pub top: i32,
    /// The Left Position of the Region
    pub left: i32,
    /// The Width of the Region
    pub width: i32,
    /// The Height of the Region
    pub height: i32,
    /// If the Region is Selected
    pub selected: bool,
    /// The ID of the Region
    pub id: String,
}
