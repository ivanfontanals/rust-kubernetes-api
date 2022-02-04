use crate::domain::model::{NodeGroupDto, SecretDto};
use anyhow::Result;
use k8s_openapi::api::core::v1::Secret;
use std::convert::TryFrom;

use crate::infrastructure::kubernetes::model::NodeGroup;
use kube::ResourceExt;

impl TryFrom<Secret> for SecretDto {
  type Error = anyhow::Error;

  fn try_from(d: Secret) -> Result<Self> {
    let name = d.name();
    let namespace = d.namespace().unwrap();
    Ok(SecretDto {
      name,
      namespace,
      version: "1".to_string(),
    })
  }
}

impl TryFrom<NodeGroup> for NodeGroupDto {
  type Error = anyhow::Error;

  fn try_from(ng: NodeGroup) -> Result<Self> {
    let name = ng.name();
    let instance_name = ng.spec.instance_types.map(|instance_types| instance_types.default);
    let ephemeral = ng.spec.storage.and_then(|storage| storage.ephemeral);

    let min_size = ng.spec.size.as_ref().and_then(|size| size.min);
    let max_size = ng.spec.size.as_ref().and_then(|size| size.max);
    let state = ng.status.and_then(|status| status.state).map(|status| status.to_string());

    Ok(NodeGroupDto {
      name,
      instance_name,
      min_size,
      max_size,
      ephemeral,
      state,
      instance_type: None,
    })
  }
}
