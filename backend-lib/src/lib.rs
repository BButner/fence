use backend::update_cursor_location;
use tokio::runtime::Runtime;

static mut RUNTIME: Option<Runtime> = None;

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
pub extern "C" fn update_mouse_location(x: i32, y: i32) {
    update_cursor_location(x, y);
}
