use async_trait::async_trait;

#[async_trait(?Send)]
pub trait ProbesService: Send {
  fn is_ready(&self) -> bool;
}

#[cfg(test)]
pub mod tests {
  use async_trait::async_trait;

  use crate::domain::ports::incoming::probes::ProbesService;

  pub struct ProbesServiceMock {
    pub is_ready: bool,
  }

  impl ProbesServiceMock {
    #[allow(unused)]
    pub fn ready() -> ProbesServiceMock {
      Self { is_ready: true }
    }

    #[allow(unused)]
    pub fn not_ready() -> ProbesServiceMock {
      Self { is_ready: false }
    }
  }

  #[async_trait(?Send)]
  impl ProbesService for ProbesServiceMock {
    fn is_ready(&self) -> bool {
      self.is_ready
    }
  }
}
