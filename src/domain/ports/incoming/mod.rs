pub mod probes;
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;

use crate::domain::model::{InstanceType, NodegroupRequestDto, SecretDto, SecretRequestDto};
use crate::domain::ports::outgoing::DataSource;

#[async_trait(?Send)]
pub trait ReadService<K>: Send
where
  K: Serialize,
{
  fn get(&self, name: &str) -> Option<K>;
  fn list(&self) -> Option<Vec<K>>;
}

#[async_trait(?Send)]
pub trait NodegroupService<K>: Send
where
  K: Serialize,
{
  fn get(&self, name: &str) -> Option<K>;
  fn list(&self) -> Option<Vec<K>>;
  fn create(&self, request: &NodegroupRequestDto) -> Result<(), anyhow::Error>;
}

/// Trait to define a Template service

pub trait SecretService {
  fn get(&self, name: &str) -> Option<SecretDto>;
  fn list(&self) -> Option<Vec<SecretDto>>;
  fn create(&self, request: &SecretRequestDto) -> Result<(), anyhow::Error>;
  fn render(&self, request: &SecretRequestDto) -> Result<String, anyhow::Error>;
}

pub trait TemplateService {
  /// Returns a Future with the rendered template
  ///
  /// # Arguments
  ///
  /// * `name` - A string that holds the name of template to be applied
  /// * `values` - Map with all the key/values to be replaced in the template
  ///
  fn render(&self, name: &str, values: &HashMap<&str, &str>) -> Result<String, anyhow::Error>;

  /// Writes a rendered template to the file_path destination
  ///
  /// # Arguments
  ///
  /// * `name` - A string that holds the name of template to be applied
  /// * `values` - Map with all the key/values to be replaced in the template
  ///
  fn write_to_file(&self, name: &str, values: &HashMap<&str, &str>, file_path: &str) -> Result<(), Error>;
}

pub trait WithName {
  fn name(&self) -> String;
}

#[async_trait(?Send)]
pub trait ScheduledService: Send {
  fn start(self) -> Result<()>;
}
#[async_trait(?Send)]
pub trait InstanceTypesUpdater: Send {
  fn execute<D>(&mut self, data_source: &D) -> Result<usize>
  where
    D: DataSource;
}

pub trait InstanceTypesService: Send {
  fn list(&self) -> Result<Vec<InstanceType>>;
}
