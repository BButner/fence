use anyhow::Result;
use tauri::Manager;

pub mod displays;
pub mod regions;

use crate::{
    events::{grpc_status, CursorPositionPayload, EventPayload, CURSOR_POSITION, GRPC_STATUS},
    state::{FenceState, StateResponse},
};

#[tauri::command]
pub async fn get_state(state: tauri::State<'_, FenceState>) -> Result<StateResponse, ()> {
    println!("get_state");
    let state = state.0.lock().await;
    println!("after lock");
    Ok(state.into())
}

#[tauri::command]
pub async fn connect_grpc(
    hostname: String,
    state: tauri::State<'_, FenceState>,
    window: tauri::Window,
) -> Result<(), ()> {
    println!("connect_grpc");
    let mut state = state.0.lock().await;

    state.current_hostname = Some(hostname.clone());

    let _ = window.app_handle().emit_all(
        GRPC_STATUS,
        EventPayload::new(grpc_status::CONNECTING.to_string(), "".to_string()),
    );
    state.grpc_status = grpc_status::CONNECTING.to_string();

    match crate::grpc::connect_client(&hostname).await {
        Ok(client) => {
            let mut heartbeat_client = client.client.clone();
            let mut cursor_client = client.client.clone();

            let cursor_handle = window.app_handle().clone();

            tokio::spawn(async move {
                let stream = cursor_client.get_cursor_location(()).await;

                match stream {
                    Ok(stream) => {
                        // get all the messages from the stream
                        let mut stream = stream.into_inner();

                        loop {
                            match stream.message().await {
                                Ok(Some(pos)) => {
                                    let _ = cursor_handle.emit_all(
                                        CURSOR_POSITION,
                                        CursorPositionPayload { x: pos.x, y: pos.y },
                                    );
                                }
                                Ok(None) => {
                                    break;
                                }
                                Err(e) => {
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error getting cursor stream");
                    }
                }
            });

            state.current_client = Some(client);
            let _ = window.app_handle().emit_all(
                GRPC_STATUS,
                EventPayload::new(grpc_status::CONNECTED.to_string(), hostname),
            );
            state.grpc_status = grpc_status::CONNECTED.to_string();

            tokio::spawn(async move {
                let stream = heartbeat_client.get_heartbeat(()).await;

                match stream {
                    Ok(stream) => {
                        // get all the messages from the stream
                        let mut stream = stream.into_inner();

                        loop {
                            match stream.message().await {
                                Ok(Some(_)) => {}
                                Ok(None) => {
                                    let _ = window.app_handle().emit_all(
                                        GRPC_STATUS,
                                        EventPayload::new(
                                            grpc_status::CONNECTION_LOST.to_string(),
                                            "Connection lost".to_string(),
                                        ),
                                    );
                                    break;
                                }
                                Err(e) => {
                                    let _ = window.app_handle().emit_all(
                                        GRPC_STATUS,
                                        EventPayload::new(
                                            grpc_status::CONNECTION_LOST.to_string(),
                                            e.to_string(),
                                        ),
                                    );
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error getting heartbeat stream");
                    }
                }
            });

            Ok(())
        }
        Err(e) => {
            println!("{:?}", e);

            let _ = window.app_handle().emit_all(
                GRPC_STATUS,
                EventPayload::new(grpc_status::ERROR.to_string(), e.to_string()),
            );
            state.grpc_status = grpc_status::ERROR.to_string();

            Err(())
        }
    }
}
