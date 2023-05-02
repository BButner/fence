use serde::{Deserialize, Serialize};

use crate::{events::grpc_status, state::FenceState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    id: String,
}

#[tauri::command]
pub async fn get_regions(state: tauri::State<'_, FenceState>) -> Result<Vec<Region>, ()> {
    println!("get_regions");
    let mut state = state.0.lock().await;

    if state.grpc_status != grpc_status::CONNECTED {
        return Ok(vec![]);
    }

    let raw_regions = state
        .current_client
        .as_mut()
        .unwrap()
        .client
        .get_regions(())
        .await;

    match raw_regions {
        Ok(regions) => {
            let regions_copy = regions.get_ref().clone();

            Ok(regions
                .get_ref()
                .regions
                .iter()
                .map(|r| Region {
                    width: r.width,
                    height: r.height,
                    x: r.x,
                    y: r.y,
                    id: r.id.clone(),
                })
                .collect())
        }
        Err(e) => {
            println!("Error getting regions: {:?}", e);
            Ok(vec![])
        }
    }
}

// impl From<&crate::grpc::fence::Region> for Region {
//     fn from(region: &crate::grpc::fence::Region) -> Self {
//         Region {
//             width: region.width,
//             height: region.height,
//             x: region.x,
//             y: region.y,
//             id: region.id.clone(),
//         }
//     }
// }
