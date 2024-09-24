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
