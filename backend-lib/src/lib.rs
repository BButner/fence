use std::sync::Arc;

use backend::grpc::State;
use tokio::{runtime::Runtime, sync::Mutex};

static mut RUNTIME: Option<Runtime> = None;
static mut STATE: Option<Arc<Mutex<State>>> = None;
static mut GET_DISPLAYS_FUNC: Option<extern "C" fn() -> DisplayArray> = None;

#[repr(C)]
pub struct MouseLocation {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct UpdateMouseLocationResult {
    pub updated: bool,
    pub location: MouseLocation,
}

#[derive(Debug)]
#[repr(C)]
pub struct DisplayArray {
    pub displays: *mut Display,
    pub array_length: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Display {
    pub width: i32,
    pub height: i32,
    pub top: i32,
    pub left: i32,
    pub is_primary: bool,
}

pub fn get_displays_wrapper() -> Vec<backend::display::Display> {
    let raw_result = unsafe { GET_DISPLAYS_FUNC.unwrap()() };

    let displays = unsafe {
        std::slice::from_raw_parts(
            raw_result.displays,
            raw_result.array_length.try_into().unwrap(),
        )
    };

    displays
        .iter()
        .map(|display| backend::display::Display {
            width: display.width,
            height: display.height,
            top: display.top,
            left: display.left,
            is_primary: display.is_primary,
        })
        .collect()
}

#[no_mangle]
pub extern "C" fn init_fence(get_displays_fn: extern "C" fn() -> DisplayArray) -> bool {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    unsafe { GET_DISPLAYS_FUNC = Some(get_displays_fn) }

    match runtime {
        Ok(runtime) => unsafe {
            RUNTIME = Some(runtime);

            let result = RUNTIME
                .as_ref()
                .unwrap()
                .block_on(backend::init_fence(get_displays_wrapper));

            match result {
                Some(state) => {
                    STATE = Some(state);
                    true
                }
                None => false,
            }
        },
        Err(e) => {
            println!("Error creating runtime: {:?}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn try_update_mouse_location(x: i32, y: i32) -> UpdateMouseLocationResult {
    let state = unsafe { STATE.as_ref().unwrap() };

    let result = state.blocking_lock().try_update_cursor_location(x, y);

    UpdateMouseLocationResult {
        updated: result.updated,
        location: MouseLocation {
            x: result.location.x,
            y: result.location.y,
        },
    }
}
