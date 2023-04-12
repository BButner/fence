use std::sync::Arc;

use grpc::State;
use tokio::sync::Mutex;

pub mod config;
pub mod cursor;
pub mod grpc;
pub mod region;

pub async fn init_fence() -> Option<Arc<Mutex<State>>> {
    println!("Initializing gRPC connection");
    grpc::init_connection().await
}
