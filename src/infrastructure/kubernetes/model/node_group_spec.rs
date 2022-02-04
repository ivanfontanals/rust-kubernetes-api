use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
#[kube(
  group = "cluster.unicron.mpi-internal.com",
  version = "v1alpha1",
  kind = "NodeGroup",
  status = "NodeGroupStatus",
  derive = "PartialEq"
)]
/// Representation of a Node Group spec
pub struct NodeGroupSpec {
  pub size: Option<NodeGroupSize>,
  pub storage: Option<NodeGroupStorage>,
  #[serde(alias = "instanceTypes")]
  pub instance_types: Option<NodeGroupInstanceTypes>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NodeGroupInstanceTypes {
  pub default: String,
  pub alternates: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NodeGroupStorage {
  pub ephemeral: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NodeGroupSize {
  pub max: Option<usize>,
  pub min: Option<usize>,
  pub target: Option<usize>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NodeGroupStatus {
  pub state: Option<NodeGroupState>,
}

#[derive(Serialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum NodeGroupState {
  Present,
  Absent,
  Processing,
  Invalid,
  Unknown(String),
}

impl<'de> Deserialize<'de> for NodeGroupState {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?.to_lowercase();
    Ok(NodeGroupState::from(s))
  }
}

impl std::fmt::Display for NodeGroupState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<String> for NodeGroupState {
  fn from(value: String) -> Self {
    match value.to_lowercase().as_str() {
      "present" => NodeGroupState::Present,
      "absent" => NodeGroupState::Absent,
      "processing" => NodeGroupState::Processing,
      "invalid" => NodeGroupState::Invalid,
      _ => NodeGroupState::Unknown(value),
    }
  }
}
