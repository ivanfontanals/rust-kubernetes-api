use crate::domain::ports::incoming::probes::ProbesService;
use async_trait::async_trait;

pub struct DefaultProbesService {}

impl DefaultProbesService {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait(?Send)]
impl ProbesService for DefaultProbesService {
  fn is_ready(&self) -> bool {
    true
  }
}
