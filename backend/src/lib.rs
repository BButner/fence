use std::sync::{Arc, Mutex};

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

static REGIONS: Arc<Mutex<Vec<Region>>> = Arc::new(Mutex::new(Vec::new()));

static LAST_GOOD_POS: Arc<Mutex<Option<CursorLocation>>> = Arc::new(Mutex::new(None));

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

pub struct UpdateCursorLocationResult {
    pub updated: bool,
    pub location: CursorLocation,
}

pub fn try_update_cursor_location(x: i32, y: i32) -> UpdateCursorLocationResult {
    let tx = TX.lock().unwrap();
    let regions = unsafe { REGIONS.lock().as_ref().unwrap() };
    let last_good_pos = unsafe { LAST_GOOD_POS.lock().as_mut().unwrap() };

    if last_good_pos.is_none() {
        last_good_pos.replace(CursorLocation { x, y });
    }

    if let Some(last_good_pos) = last_good_pos.as_mut() {
        let mut inside_region = false;

        for region in regions.iter() {
            if region.is_inside(x, y, 0) {
                inside_region = true;
                println!("Cursor is within region: {}", region.id);
                break;
            }
        }

        if inside_region {
            let current_region = regions.iter().find(|region| region.is_inside(x, y, 1));

            if let Some(region) = current_region {
                let mut new_x = x;
                let mut new_y = y;

                let mut new_x_valid = true;
                for region in regions.iter() {
                    if region.is_inside(new_x, y, 0) {
                        new_x_valid = false;
                        break;
                    }
                }

                let mut new_y_valid = true;
                for region in regions.iter() {
                    if region.is_inside(x, new_y, 0) {
                        new_y_valid = false;
                        break;
                    }
                }

                if !new_x_valid && !new_y_valid {
                    new_x = last_good_pos.x;
                    new_y = last_good_pos.y;
                } else if !new_x_valid {
                    new_x = last_good_pos.x;
                } else if !new_y_valid {
                    new_y = last_good_pos.y;
                }

                if let Some(tx) = tx.as_ref() {
                    let _ = tx.send(CursorLocation { x: new_x, y: new_y });
                }

                UpdateCursorLocationResult {
                    updated: false,
                    location: CursorLocation { x: new_x, y: new_y },
                }
            } else {
                println!("Failed to find region for cursor location");

                if let Some(tx) = tx.as_ref() {
                    let _ = tx.send(CursorLocation { x: x, y: y });
                }

                unsafe {
                    UpdateCursorLocationResult {
                        updated: false,
                        location: CursorLocation {
                            x: last_good_pos.x,
                            y: last_good_pos.y,
                        },
                    }
                }
            }
        } else {
            unsafe {
                last_good_pos.x = x;
                last_good_pos.y = y;

                if let Some(tx) = tx.as_ref() {
                    let _ = tx.send(CursorLocation { x: x, y: y });
                }

                UpdateCursorLocationResult {
                    updated: true,
                    location: CursorLocation { x, y },
                }
            }
        }
    } else {
        println!("Failed to get last good position");

        if let Some(tx) = tx.as_ref() {
            let _ = tx.send(CursorLocation { x: x, y: y });
        }

        UpdateCursorLocationResult {
            updated: false,
            location: CursorLocation { x, y },
        }
    }
}
