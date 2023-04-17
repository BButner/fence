use crate::grpc::fence;

#[derive(Debug)]
pub struct Display {
    width: i32,
    height: i32,
    top: i32,
    left: i32,
    is_primary: bool,
}

impl From<fence::Display> for Display {
    fn from(display: fence::Display) -> Self {
        Self {
            width: display.width,
            height: display.height,
            top: display.top,
            left: display.left,
            is_primary: display.is_primary,
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
        }
    }
}
