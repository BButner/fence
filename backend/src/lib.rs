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

pub fn update_cursor_location(x: i32, y: i32) {
    let tx = TX.lock().unwrap();

    // check if the location is within a region, comparing the x and the y coordinates with the width and the height
    // of the region
    for region in unsafe { &REGIONS } {
        if x >= region.x
            && x <= region.x + region.width
            && y >= region.y
            && y <= region.y + region.height
        {
            println!("Cursor is within region: {}", region.id);
        }
    }

    if let Some(tx) = &*tx {
        let _ = tx.send(CursorLocation { x, y });
    }
}
