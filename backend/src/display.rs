use crate::grpc::fence;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Display {
    pub width: i32,
    pub height: i32,
    pub top: i32,
    pub left: i32,
    pub is_primary: bool,
    pub screen_data: String,
}

impl From<fence::Display> for Display {
    fn from(display: fence::Display) -> Self {
        Self {
            width: display.width,
            height: display.height,
            top: display.top,
            left: display.left,
            is_primary: display.is_primary,
            screen_data: display.screen_data,
        }
    }
}

impl From<&Display> for fence::Display {
    fn from(display: &Display) -> Self {
        Self {
            width: display.width,
            height: display.height,
            top: display.top,
            left: display.left,
            is_primary: display.is_primary,
            screen_data: display.screen_data.clone(),
        }
    }
}
