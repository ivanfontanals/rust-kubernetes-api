use std::io::Read;

use crate::domain::ports::outgoing::DataSource;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct UrlDataSource {
  url: String,
}

impl UrlDataSource {
  #[allow(unused)]
  pub fn new<S: Into<String>>(url: S) -> Self {
    Self { url: url.into() }
  }
}

impl DataSource for UrlDataSource {
  fn name(&self) -> &str {
    "url"
  }

  fn reader<'a>(&'a self) -> Result<Box<dyn Read + 'a>> {
    ureq::get(self.url.as_str())
      .call()
      .map(|response| Box::new(response.into_reader()) as Box<dyn Read>)
      .map_err(anyhow::Error::from)
  }
}
