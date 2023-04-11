use backend::try_update_cursor_location;
use tokio::runtime::Runtime;

static mut RUNTIME: Option<Runtime> = None;

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

            result
        },
        Err(e) => {
            println!("Error creating runtime: {:?}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn try_update_mouse_location(x: i32, y: i32) -> UpdateMouseLocationResult {
    let result = try_update_cursor_location(x, y);

    println!("Original: x: {}, y: {}", x, y);
    println!("Result: x: {}, y: {}", result.location.x, result.location.y);

    UpdateMouseLocationResult {
        updated: result.updated,
        location: MouseLocation {
            x: result.location.x,
            y: result.location.y,
        },
    }
}
