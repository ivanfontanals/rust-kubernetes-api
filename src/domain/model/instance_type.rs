use crate::domain::ports::incoming::WithName;
use crate::domain::services::instance_types::deserializer::deserialize_instance_types;
use serde_derive::{Deserialize, Serialize};

/// An instance type description
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InstanceType {
  /// Instance name
  pub name: String,

  /// Instance family
  pub family: String,

  /// Memory in bytes
  pub memory: usize,

  /// Number of virtual CPUs
  pub vcpu: usize,

  /// Number of GPUs
  pub gpu: usize,
}

impl WithName for InstanceType {
  fn name(&self) -> String {
    self.name.clone()
  }
}
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct InstanceTypesList {
  pub version: String,

  #[serde(deserialize_with = "deserialize_instance_types")]
  #[serde(rename = "products")]
  pub instance_types: Vec<InstanceType>,
}
