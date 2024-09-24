use serde::{Deserialize, Serialize};

pub const HANDSHAKE_COMMAND: &str = "handshake";
pub const HEARTBEAT_EVENT: &str = "heartbeat";
pub const VERSION_NAME_MESSAGE: &str = "version_info";
pub const ERROR_EVENT: &str = "error";

pub const K8S_GET_VERSION_COMMAND: &str = "kubernetes_get_version";
pub const K8S_GET_VERSION_EVENT: &str = "kubernetes_version";

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
  pub name: String,
}

