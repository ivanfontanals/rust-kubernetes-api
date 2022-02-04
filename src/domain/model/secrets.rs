use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct SecretDto {
  pub name: String,
  pub namespace: String,
  pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SecretRequestDto {
  pub name: String,
  pub literals: HashMap<String, String>,
  pub skip_pull_request: bool,
}
