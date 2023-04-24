use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;

use crate::grpc::GrpcClient;

pub struct FenceState(pub Mutex<State>);

impl FenceState {
    pub fn new() -> Self {
        Self(Mutex::new(State {
            current_hostname: None,
            current_client: None,
            grpc_status: String::new(),
        }))
    }
}

pub struct State {
    pub current_hostname: Option<String>,
    pub current_client: Option<GrpcClient>,
    pub grpc_status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateResponse {
    pub current_hostname: Option<String>,
    pub grpc_status: String,
}

impl From<tokio::sync::MutexGuard<'_, State>> for StateResponse {
    fn from(state: tokio::sync::MutexGuard<'_, State>) -> Self {
        Self {
            current_hostname: state.current_hostname.clone(),
            grpc_status: state.grpc_status.clone(),
        }
    }
}
