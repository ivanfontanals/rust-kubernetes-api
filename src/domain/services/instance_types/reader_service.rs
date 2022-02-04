use anyhow::Result;

use crate::domain::model::InstanceType;
use crate::domain::ports::incoming::InstanceTypesService;
use crate::domain::ports::outgoing::ReadStore;

pub struct DefaultInstanceTypesService<S> {
  store: S,
}

impl<S> DefaultInstanceTypesService<S>
where
  S: ReadStore<InstanceType> + Send + Sync + 'static,
{
  pub fn new(store: S) -> Self {
    Self { store }
  }
}

impl<S> InstanceTypesService for DefaultInstanceTypesService<S>
where
  S: ReadStore<InstanceType> + Send + Sync + 'static,
{
  fn list(&self) -> Result<Vec<InstanceType>> {
    self.store.list()
  }
}
