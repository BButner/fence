use std::{
    sync::{Arc, Mutex},
    thread::current,
};

use grpc::fence::CursorLocation;
use once_cell::sync::Lazy;
use region::Region;

pub mod config;
pub mod grpc;
pub mod region;

pub struct State {
    current_config: Option<config::Config>,
}

impl State {
    pub fn default() -> Self {
        Self {
            current_config: None,
        }
    }
}

static TX: Lazy<Arc<Mutex<Option<tokio::sync::broadcast::Sender<CursorLocation>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

static STATE: Lazy<Arc<tokio::sync::Mutex<State>>> =
    Lazy::new(|| Arc::new(tokio::sync::Mutex::new(State::default())));

static mut REGIONS: Vec<Region> = Vec::new();

pub async fn init_fence() -> bool {
    let config = config::Config::load(None).await;

    match config {
        Some(config) => {
            let mut state = STATE.lock().await;
            state.current_config = Some(config);
        }
        None => {
            println!("Failed to load config");
            return false;
        }
    }

    println!("Initializing gRPC connection");
    let grpc_tx = grpc::init_connection().await;

    let mut tx = TX.lock().unwrap();
    *tx = Some(grpc_tx);
    true
}

static mut LAST_GOOD_POS: Option<CursorLocation> = None;

pub struct UpdateCursorLocationResult {
    pub updated: bool,
    pub location: CursorLocation,
}

pub fn try_update_cursor_location(x: i32, y: i32) -> UpdateCursorLocationResult {
    let tx = TX.lock().unwrap();

    let mut inside_region = false;

    for region in unsafe { &REGIONS } {
        if region.is_inside(x, y, 0) {
            inside_region = true;
            println!("Cursor is within region: {}", region.id);
            break;
        }
    }

    if inside_region {
        let current_region = unsafe { REGIONS.iter().find(|region| region.is_inside(x, y, 1)) };

        if let Some(region) = current_region {
            let mut new_x = x;
            let mut new_y = y;

            // check if new_x, and original y is valid
            // check if new_y, and original x is valid
            // If neither, then we need to move both

            // Check if any regions are within the new x, and original y
            let mut new_x_valid = true;
            for region in unsafe { &REGIONS } {
                if region.is_inside(new_x, y, 0) {
                    new_x_valid = false;
                    break;
                }
            }

            // Check if any regions are within the new y, and original x
            let mut new_y_valid = true;
            for region in unsafe { &REGIONS } {
                if region.is_inside(x, new_y, 0) {
                    new_y_valid = false;
                    break;
                }
            }

            if !new_x_valid && !new_y_valid {
                // Move both
                new_x = unsafe { LAST_GOOD_POS.as_ref().unwrap().x };
                new_y = unsafe { LAST_GOOD_POS.as_ref().unwrap().y };
            } else if !new_x_valid {
                // Move x
                new_x = unsafe { LAST_GOOD_POS.as_ref().unwrap().x };
            } else if !new_y_valid {
                // Move y
                new_y = unsafe { LAST_GOOD_POS.as_ref().unwrap().y };
            }

            UpdateCursorLocationResult {
                updated: false,
                location: CursorLocation { x: new_x, y: new_y },
            }
        } else {
            println!("Failed to find region for cursor location");
            unsafe {
                UpdateCursorLocationResult {
                    updated: false,
                    location: CursorLocation {
                        x: LAST_GOOD_POS.as_ref().unwrap().x,
                        y: LAST_GOOD_POS.as_ref().unwrap().y,
                    },
                }
            }
        }
    } else {
        unsafe {
            LAST_GOOD_POS = Some(CursorLocation { x, y });

            UpdateCursorLocationResult {
                updated: true,
                location: CursorLocation { x, y },
            }
        }
    }
}
