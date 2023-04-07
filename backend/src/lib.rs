use std::sync::{Arc, Mutex};

use grpc::fence::CursorLocation;
use once_cell::sync::Lazy;

pub mod config;
pub mod grpc;
pub mod region;
pub mod tests;

static TX: Lazy<Arc<Mutex<Option<tokio::sync::broadcast::Sender<CursorLocation>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn init_fence() -> bool {
    let config = config::Config::load(None).await;

    match config {
        Some(config) => {
            println!("Loaded config: {:?}", config);
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

    if let Some(tx) = &*tx {
        let _ = tx.send(CursorLocation { x, y });
    }
}
