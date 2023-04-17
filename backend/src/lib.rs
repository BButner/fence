use std::sync::Arc;

use display::Display;
use grpc::State;
use tokio::sync::Mutex;

pub mod config;
pub mod cursor;
pub mod display;
pub mod grpc;
pub mod region;

pub async fn init_fence(get_displays_fn: fn() -> Vec<Display>) -> Option<Arc<Mutex<State>>> {
    println!("Initializing gRPC connection");
    grpc::init_connection(get_displays_fn).await
}
