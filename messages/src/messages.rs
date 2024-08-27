use serde::{Deserialize, Serialize};

pub const HANDSHAKE_COMMAND: &str = "handshake";
pub const HEARTBEAT_EVENT: &str = "heartbeat";
pub const VERSION_NAME_MESSAGE: &str = "version_info";

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
}
