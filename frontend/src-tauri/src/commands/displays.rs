use serde::{Deserialize, Serialize};

use crate::{events::grpc_status, state::FenceState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    width: i32,
    height: i32,
    top: i32,
    left: i32,
    is_primary: bool,
}

#[tauri::command]
pub async fn get_displays(state: tauri::State<'_, FenceState>) -> Result<Vec<Display>, ()> {
    let mut state = state.0.lock().await;

    if state.grpc_status != grpc_status::CONNECTED {
        return Ok(vec![]);
    }

    let raw_displays = state
        .current_client
        .as_mut()
        .unwrap()
        .client
        .get_displays(())
        .await;

    match raw_displays {
        Ok(displays) => Ok(displays
            .get_ref()
            .displays
            .iter()
            .map(|d| d.into())
            .collect()),
        Err(e) => {
            println!("Error getting displays: {:?}", e);
            Ok(vec![])
        }
    }
}

impl From<&crate::grpc::fence::Display> for Display {
    fn from(display: &crate::grpc::fence::Display) -> Self {
        Display {
            width: display.width,
            height: display.height,
            left: display.left,
            top: display.top,
            is_primary: display.is_primary,
        }
    }
}
