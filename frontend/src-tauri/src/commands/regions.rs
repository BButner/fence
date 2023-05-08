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
    let mut raw_regions;

    {
        let mut state = state.0.lock().await;

        if state.grpc_status != grpc_status::CONNECTED {
            return Ok(vec![]);
        }

        let regions_response = state
            .current_client
            .as_mut()
            .unwrap()
            .client
            .get_regions(())
            .await;

        match regions_response {
            Ok(regions) => {
                raw_regions = regions
                    .get_ref()
                    .regions
                    .iter()
                    .map(|region| Region {
                        width: region.width,
                        height: region.height,
                        x: region.x,
                        y: region.y,
                        id: region.id.clone(),
                    })
                    .collect();
            }
            Err(e) => {
                println!("Error getting regions: {:?}", e);
                return Ok(vec![]);
            }
        }
    }

    Ok(raw_regions)
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
