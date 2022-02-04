use std::{io::Read, path::Path};

use crate::domain::ports::outgoing::DataSource;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FileDataSource {
  path: String,
}

impl FileDataSource {
  pub fn new(path: String) -> Self {
    Self { path }
  }
}

impl DataSource for FileDataSource {
  fn name(&self) -> &str {
    "file"
  }

  fn reader<'a>(&'a self) -> Result<Box<dyn Read + 'a>> {
    std::fs::File::open(Path::new(self.path.as_str()))
      .map(|file| Box::new(file) as Box<dyn Read>)
      .map_err(anyhow::Error::from)
  }
}
