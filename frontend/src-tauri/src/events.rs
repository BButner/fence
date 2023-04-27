use serde::{Deserialize, Serialize};

pub mod grpc_status {
    pub const CONNECTING: &str = "CONNECTING";
    pub const CONNECTED: &str = "CONNECTED";
    pub const DISCONNECTED: &str = "DISCONNECTED";
    pub const ERROR: &str = "ERROR";
    pub const CONNECTION_LOST: &str = "CONNECTION_LOST";
    pub const HEARTBEAT: &str = "HEARTBEAT";
}

pub const GRPC_STATUS: &str = "EVENT_GRPC_STATUS";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventPayload {
    pub event: String,
    pub payload: String,
}

impl EventPayload {
    pub fn new(event: String, payload: String) -> Self {
        Self { event, payload }
    }
}
