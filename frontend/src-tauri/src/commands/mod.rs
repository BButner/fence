use anyhow::Result;
use tauri::Manager;

use crate::{
    events::{grpc_status, EventPayload},
    state::{FenceState, StateResponse},
};

#[tauri::command]
pub async fn get_state(state: tauri::State<'_, FenceState>) -> Result<StateResponse, ()> {
    let state = state.0.lock().await;
    Ok(state.into())
}

#[tauri::command]
pub async fn connect_grpc(
    hostname: String,
    state: tauri::State<'_, FenceState>,
    window: tauri::Window,
) -> Result<(), ()> {
    let mut state = state.0.lock().await;

    state.current_hostname = Some(hostname.clone());

    let _ = window.app_handle().emit_all(
        grpc_status::CONNECTING,
        EventPayload::new(grpc_status::CONNECTING.to_string(), "".to_string()),
    );
    state.grpc_status = grpc_status::CONNECTING.to_string();

    match crate::grpc::connect_client(&hostname).await {
        Ok(client) => {
            state.current_client = Some(client);
            let _ = window.app_handle().emit_all(
                grpc_status::CONNECTED,
                EventPayload::new(grpc_status::CONNECTED.to_string(), hostname),
            );
            state.grpc_status = grpc_status::CONNECTED.to_string();
        }
        Err(e) => {
            let _ = window.app_handle().emit_all(
                grpc_status::ERROR,
                EventPayload::new(grpc_status::ERROR.to_string(), e.to_string()),
            );
            state.grpc_status = grpc_status::ERROR.to_string();
        }
    }

    Ok(())
}
