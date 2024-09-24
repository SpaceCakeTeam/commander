use serde::{Deserialize, Serialize};
use kube::Client;
use messages::error::Error;

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

pub async fn get_version() -> Result<KubernetesVersion, Error> {
  let client = Client::try_default().await
    .map_err(|e| Error { message: e.to_string() })?;
  client.apiserver_version().await
    .map(|v| KubernetesVersion {
      major: v.major,
      minor: v.minor,
      git_version: v.git_version,
      git_commit: v.git_commit,
      git_tree_state: v.git_tree_state,
      build_date: v.build_date,
      go_version: v.go_version,
      compiler: v.compiler,
      platform: v.platform,
    })
    .map_err(|e| Error { message: e.to_string() })
}

#[cfg(test)]
mod kube_tests {
  use super::*;

  #[test]
  fn test_kubernetes_version_struct_serialization() {
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