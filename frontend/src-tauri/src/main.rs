#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod commands;
pub mod events;
pub mod grpc;
pub mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .manage(state::FenceState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::connect_grpc,
            commands::displays::get_displays,
            commands::displays::get_display_screenshots,
            commands::regions::get_regions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
