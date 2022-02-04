use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpdaterError {
  #[error("Error reading from the data source: {0}")]
  ReadDataSource(#[source] anyhow::Error),

  #[error("Error updating store: {0}")]
  UpdateStore(#[source] anyhow::Error),
}
