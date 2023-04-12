use std::sync::Arc;

use backend::grpc::State;
use tokio::{runtime::Runtime, sync::Mutex};

static mut RUNTIME: Option<Runtime> = None;
static mut STATE: Option<Arc<Mutex<State>>> = None;

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

#[no_mangle]
pub extern "C" fn init_fence() -> bool {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    match runtime {
        Ok(runtime) => unsafe {
            RUNTIME = Some(runtime);

            let result = RUNTIME.as_ref().unwrap().block_on(backend::init_fence());

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
