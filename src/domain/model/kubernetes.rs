use crate::domain::model::InstanceType;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Clone, Debug, PartialEq, Eq)]

pub struct ResponseStatusDto {
  pub status: String,
}
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]

pub struct NodeGroupDto {
  pub name: String,
  pub instance_name: Option<String>,
  pub min_size: Option<usize>,
  pub max_size: Option<usize>,
  pub ephemeral: Option<String>,
  pub state: Option<String>,
  pub instance_type: Option<InstanceType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NodegroupRequestDto {
  pub name: String,
  pub default_instance_type: String,
  pub alternate_instance_type: Option<String>,
  pub min_size: Option<String>,
  pub max_size: Option<String>,
  pub target_size: Option<String>,
  pub ephemeral: String,
  pub skip_pull_request: bool,
}
