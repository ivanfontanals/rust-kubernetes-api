use anyhow::Result;
use log::info;

use crate::domain::model::{InstanceType, InstanceTypesList};
use crate::domain::ports::incoming::InstanceTypesUpdater;
use crate::domain::ports::outgoing::{DataSource, WriteStore};
use crate::domain::services::instance_types::UpdaterError;

pub struct DefaultInstanceTypesUpdater<S> {
  pub last_version: Option<String>,
  store: S,
}

impl<S> DefaultInstanceTypesUpdater<S>
where
  S: WriteStore<InstanceType> + Send + Sync + 'static,
{
  pub fn new(store: S) -> Self {
    Self { last_version: None, store }
  }
}

impl<S> InstanceTypesUpdater for DefaultInstanceTypesUpdater<S>
where
  S: WriteStore<InstanceType> + Send + Sync + 'static,
{
  fn execute<D>(&mut self, data_source: &D) -> Result<usize>
  where
    D: DataSource,
  {
    let reader = data_source.reader().map_err(UpdaterError::ReadDataSource)?;
    let pricing_list: InstanceTypesList = serde_json::from_reader(reader).map_err(anyhow::Error::from)?;

    let load_count = pricing_list.instance_types.len();

    let updated_version = self
      .last_version
      .as_ref()
      .map_or(true, |last_version| *last_version != pricing_list.version);

    if updated_version {
      info!("Version has changed, so we will update the store right now.");
      let instance_types = pricing_list
        .instance_types
        .into_iter()
        .map(|product| InstanceType {
          name: product.name,
          family: product.family,
          memory: product.memory,
          vcpu: product.vcpu,
          gpu: product.gpu,
        })
        .collect();

      self.store.update(instance_types).map_err(UpdaterError::UpdateStore)?;

      self.last_version = Some(pricing_list.version);
      Ok(load_count)
    } else {
      info!("Skipping updating. Version did not changed.");
      Ok(0)
    }
  }
}
