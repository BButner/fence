use std::sync::Arc;

use grpc::State;
use tokio::sync::Mutex;

pub mod config;
pub mod grpc;
pub mod region;

pub async fn init_fence() -> Option<Arc<Mutex<State>>> {
    println!("Initializing gRPC connection");
    grpc::init_connection().await
}

// pub fn try_update_cursor_location(x: i32, y: i32) -> UpdateCursorLocationResult {
//     let tx = TX.lock().unwrap();
//     let state = STATE.lock().as_mut().unwrap();

//     if state.last_good_pos.is_none() {
//         state.last_good_pos.replace(CursorLocation { x, y });
//     }

//     if let Some(last_good_pos) = state.last_good_pos.as_mut() {
//         let mut inside_region = false;

//         for region in state.current_regions.iter() {
//             if region.is_inside(x, y, 0) {
//                 inside_region = true;
//                 println!("Cursor is within region: {}", region.id);
//                 break;
//             }
//         }

//         if inside_region {
//             let current_region = state
//                 .current_regions
//                 .iter()
//                 .find(|region| region.is_inside(x, y, 1));

//             if let Some(region) = current_region {
//                 let mut new_x = x;
//                 let mut new_y = y;

//                 let mut new_x_valid = true;
//                 for region in regions.iter() {
//                     if region.is_inside(new_x, y, 0) {
//                         new_x_valid = false;
//                         break;
//                     }
//                 }

//                 let mut new_y_valid = true;
//                 for region in regions.iter() {
//                     if region.is_inside(x, new_y, 0) {
//                         new_y_valid = false;
//                         break;
//                     }
//                 }

//                 if !new_x_valid && !new_y_valid {
//                     new_x = last_good_pos.x;
//                     new_y = last_good_pos.y;
//                 } else if !new_x_valid {
//                     new_x = last_good_pos.x;
//                 } else if !new_y_valid {
//                     new_y = last_good_pos.y;
//                 }

//                 if let Some(tx) = tx.as_ref() {
//                     let _ = tx.send(CursorLocation { x: new_x, y: new_y });
//                 }

//                 UpdateCursorLocationResult {
//                     updated: false,
//                     location: CursorLocation { x: new_x, y: new_y },
//                 }
//             } else {
//                 println!("Failed to find region for cursor location");

//                 if let Some(tx) = tx.as_ref() {
//                     let _ = tx.send(CursorLocation { x: x, y: y });
//                 }

//                 unsafe {
//                     UpdateCursorLocationResult {
//                         updated: false,
//                         location: CursorLocation {
//                             x: last_good_pos.x,
//                             y: last_good_pos.y,
//                         },
//                     }
//                 }
//             }
//         } else {
//             unsafe {
//                 last_good_pos.x = x;
//                 last_good_pos.y = y;

//                 if let Some(tx) = tx.as_ref() {
//                     let _ = tx.send(CursorLocation { x: x, y: y });
//                 }

//                 UpdateCursorLocationResult {
//                     updated: true,
//                     location: CursorLocation { x, y },
//                 }
//             }
//         }
//     } else {
//         println!("Failed to get last good position");

//         if let Some(tx) = tx.as_ref() {
//             let _ = tx.send(CursorLocation { x: x, y: y });
//         }

//         UpdateCursorLocationResult {
//             updated: false,
//             location: CursorLocation { x, y },
//         }
//     }
// }
