use crate::domain::model::NodeGroupDto;
use crate::domain::ports::incoming::{NodegroupService, TemplateService};
use crate::domain::ports::outgoing::{Repository, VersionControl};
use anyhow::Result;
use std::collections::HashMap;

use crate::domain::model::{InstanceType, NodegroupRequestDto};
use crate::domain::ports::outgoing::ReadStore;

pub struct DefaultNodegroupsService<R, S, T, V>
where
  R: Repository<NodeGroupDto>,
  S: ReadStore<InstanceType> + Send + Sync + 'static,
  T: TemplateService,
  V: VersionControl,
{
  repository: R,
  store: S,
  template_service: T,
  gitops_service: V,
}

impl<R, S, T, V> DefaultNodegroupsService<R, S, T, V>
where
  R: Repository<NodeGroupDto>,
  S: ReadStore<InstanceType> + Send + Sync + 'static,
  T: TemplateService,
  V: VersionControl,
{
  pub fn new(repository: R, store: S, template_service: T, gitops_service: V) -> Self {
    Self {
      repository,
      store,
      template_service,
      gitops_service,
    }
  }

  /*
  Adds information from the store for the instance type, if exists
  */
  pub fn populate_nodegroup(&self, nodegroup: NodeGroupDto) -> NodeGroupDto {
    let maybe_instance_name = nodegroup.clone().instance_name;
    match maybe_instance_name {
      Some(instance_name) => match self.store.get(instance_name.as_str()) {
        Ok(instance_type) => NodeGroupDto {
          instance_type: Some(instance_type),
          ..nodegroup
        },
        Err(_) => nodegroup,
      },
      _ => nodegroup,
    }
  }

  fn get_or_default<'a>(value: &'a Option<String>, default_value: &'a str) -> &'a str {
    match value {
      Some(v) => v.as_str(),
      None => default_value,
    }
  }
}

impl<R, S, T, V> NodegroupService<NodeGroupDto> for DefaultNodegroupsService<R, S, T, V>
where
  R: Repository<NodeGroupDto>,
  S: ReadStore<InstanceType> + Send + Sync + 'static,
  T: TemplateService + Send + 'static,
  V: VersionControl + Send,
{
  fn get(&self, name: &str) -> Option<NodeGroupDto> {
    self.repository.find_by(name).map(|nodegroup| self.populate_nodegroup(nodegroup))
  }

  fn list(&self) -> Option<Vec<NodeGroupDto>> {
    self.repository.find_all().map(|nodegroups| {
      nodegroups
        .into_iter()
        .map(|nodegroup| self.populate_nodegroup(nodegroup))
        .collect::<Vec<NodeGroupDto>>()
    })
  }

  fn create(&self, request: &NodegroupRequestDto) -> Result<(), anyhow::Error> {
    // Validation missing yet. PoC only

    let mut data = HashMap::new();
    data.insert("NAME", request.name.as_str());
    data.insert("EPHEMERAL_STORAGE", request.ephemeral.as_str());
    data.insert("DEFAULT_INSTANCE_TYPE", request.default_instance_type.as_str());
    data.insert("MIN_SIZE", Self::get_or_default(&request.min_size, "0"));
    data.insert("MAX_SIZE", Self::get_or_default(&request.max_size, "0"));
    data.insert("TARGET_SIZE", Self::get_or_default(&request.target_size, "0"));
    if let Some(alternate_instance_type) = &request.alternate_instance_type {
      data.insert("ALTERNATE_INSTANCE_TYPE", alternate_instance_type.as_str());
    }

    self
      .gitops_service
      .clone_repo(None)
      .and_then(|gitops_path| {
        let destination_path = format!("{}/infrastructure/_catalog/templates/nodegroup-{}.yaml", gitops_path, request.name);
        self
          .template_service
          .write_to_file("nodegroup", &data, &*destination_path)
          .map(|_| gitops_path)
      })
      .and_then(|gitops_path| match request.skip_pull_request {
        true => self
          .gitops_service
          .auto_commit(gitops_path, "Directly commited a nodegroup from Rust back-end".into()),
        false => self.gitops_service.pull_request(
          gitops_path,
          "First commit message from RUST".into(),
          "Pr Title from Rust".into(),
          "Body of the PR".into(),
          "Test_branch_from_rust".into(),
        ),
      })
  }
}
