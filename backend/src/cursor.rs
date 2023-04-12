use crate::grpc::{fence::CursorLocation, State};

pub struct UpdateCursorLocationResult {
    pub updated: bool,
    pub location: CursorLocation,
}

pub(crate) fn try_update_cursor_location(
    x: i32,
    y: i32,
    state: &mut State,
) -> UpdateCursorLocationResult {
    if state.last_good_pos.is_none() {
        state.last_good_pos.replace(CursorLocation { x, y });
    }

    if let Some(last_good_pos) = state.last_good_pos.as_mut() {
        let mut inside_region = false;

        for region in state.current_regions.iter() {
            if region.is_inside(x, y, 0) {
                inside_region = true;
                println!("Cursor is within region: {}", region.id);
                break;
            }
        }

        if inside_region {
            let current_region = state
                .current_regions
                .iter()
                .find(|region| region.is_inside(x, y, 1));

            if let Some(_) = current_region {
                let mut new_x = x;
                let mut new_y = y;

                let mut new_x_valid = true;
                for region in state.current_regions.iter() {
                    if region.is_inside(new_x, y, 0) {
                        new_x_valid = false;
                        break;
                    }
                }

                let mut new_y_valid = true;
                for region in state.current_regions.iter() {
                    if region.is_inside(x, new_y, 0) {
                        new_y_valid = false;
                        break;
                    }
                }

                if !new_x_valid && !new_y_valid {
                    new_x = last_good_pos.x;
                    new_y = last_good_pos.y;
                } else if !new_x_valid {
                    new_x = last_good_pos.x;
                } else if !new_y_valid {
                    new_y = last_good_pos.y;
                }

                let _ = state.tx.send(CursorLocation { x: new_x, y: new_y });

                UpdateCursorLocationResult {
                    updated: false,
                    location: CursorLocation { x: new_x, y: new_y },
                }
            } else {
                println!("Failed to find region for Cursor Location");

                let _ = state.tx.send(CursorLocation { x, y });

                UpdateCursorLocationResult {
                    updated: true,
                    location: CursorLocation { x, y },
                }
            }
        } else {
            let _ = state.tx.send(CursorLocation { x, y });

            UpdateCursorLocationResult {
                updated: true,
                location: CursorLocation { x, y },
            }
        }
    } else {
        println!("Failed to find last good position");

        let _ = state.tx.send(CursorLocation { x, y });

        UpdateCursorLocationResult {
            updated: true,
            location: CursorLocation { x, y },
        }
    }
}
