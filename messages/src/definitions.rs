use serde::{Deserialize, Serialize};

pub const HANDSHAKE_COMMAND: &str = "handshake";
pub const HEARTBEAT_EVENT: &str = "heartbeat";
pub const VERSION_NAME_MESSAGE: &str = "version_info";

pub const K8S_GET_VERSION_COMMAND: &str = "kubernetes_get_version";

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesVersion {
  pub major: String,
  pub minor: String,
  pub git_version: String,
  pub git_commit: String,
  pub git_tree_state: String,
  pub build_date: String,
  pub go_version: String,
  pub compiler: String,
  pub platform: String,
}

#[cfg(test)]
mod definitions_tests {
  use super::*;

  #[test]
  fn test_kubernetes_version() {
    let version = KubernetesVersion {
      major: "1".to_string(),
      minor: "20".to_string(),
      git_version: "v1.20.0".to_string(),
      git_commit: "af46c47ce925f4c4d18f44f06ed18c47d171cc1b".to_string(),
      git_tree_state: "clean".to_string(),
      build_date: "2020-12-08T17:51:19Z".to_string(),
      go_version: "go1.15.5".to_string(),
      compiler: "gc".to_string(),
      platform: "linux/amd64".to_string(),
    };

    let serialized = serde_json::to_string(&version).unwrap();

    assert_eq!(serialized, "{\"major\":\"1\",\"minor\":\"20\",\"gitVersion\":\"v1.20.0\",\"gitCommit\":\"af46c47ce925f4c4d18f44f06ed18c47d171cc1b\",\"gitTreeState\":\"clean\",\"buildDate\":\"2020-12-08T17:51:19Z\",\"goVersion\":\"go1.15.5\",\"compiler\":\"gc\",\"platform\":\"linux/amd64\"}");
  }
}
