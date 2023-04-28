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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayScreenshot {
    image_data: String,
    top: i32,
    left: i32,
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

#[tauri::command]
pub async fn get_display_screenshots(
    state: tauri::State<'_, FenceState>,
) -> Result<Vec<DisplayScreenshot>, ()> {
    let mut state = state.0.lock().await;

    if state.grpc_status != grpc_status::CONNECTED {
        return Ok(vec![]);
    }

    let screenshots = state
        .current_client
        .as_mut()
        .unwrap()
        .client
        .get_display_screenshots(())
        .await;

    match screenshots {
        Ok(screenshots) => Ok(screenshots
            .get_ref()
            .display_screenshots
            .iter()
            .map(|s| DisplayScreenshot {
                image_data: s.image_data.clone(),
                top: s.top,
                left: s.left,
            })
            .collect()),
        Err(e) => {
            println!("Error getting display screenshots: {:?}", e);
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
