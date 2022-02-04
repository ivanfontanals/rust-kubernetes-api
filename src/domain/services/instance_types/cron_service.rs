use anyhow::Result;
use log::{error, info};
use tokio::time;
use tokio::time::{Duration, Instant};

use crate::domain::ports::incoming::InstanceTypesUpdater;
use crate::domain::ports::incoming::ScheduledService;
use crate::domain::ports::outgoing::DataSource;

/// Default update interval in seconds
const UPDATE_INTERVAL_SECS: u64 = 60 * 60 * 24 * 7; // One week

/// Interval between retries in seconds
const RETRY_INTERVAL_SECS: u64 = 60 * 60; // One hour

pub struct ScheduledInstanceTypesService<FileSource, U>
where
  FileSource: DataSource + Clone + Send + 'static,
  U: InstanceTypesUpdater + Send + 'static,
{
  file_data_source: FileSource,
  updater: U,
}

impl<FileSource, U> ScheduledInstanceTypesService<FileSource, U>
where
  FileSource: DataSource + Clone + Send + 'static,
  U: InstanceTypesUpdater + Send + 'static,
{
  pub fn new(updater: U, file_data_source: FileSource) -> Self {
    Self { file_data_source, updater }
  }

  fn process(updater: &mut U, data_source: &impl DataSource) -> Duration {
    let start = Instant::now();
    match updater.execute(data_source) {
      Ok(load_count) => {
        info!(
          "Updated {} items. Elapsed time to load Instances types: {:?} seconds",
          load_count,
          start.elapsed()
        );

        info!("Next update in {} seconds", UPDATE_INTERVAL_SECS);
        Duration::from_secs(UPDATE_INTERVAL_SECS) //Normal way
      }
      Err(_) => {
        error!(
          "Failed to load the Instances types from File. Retrying in {} seconds",
          RETRY_INTERVAL_SECS
        );
        Duration::from_secs(RETRY_INTERVAL_SECS) //Retry way
      }
    }
  }
}

impl<FileSource, U> ScheduledService for ScheduledInstanceTypesService<FileSource, U>
where
  FileSource: DataSource + Clone + Send + 'static,
  U: InstanceTypesUpdater + Send + 'static,
{
  fn start(self) -> Result<()> {
    let url_datasource = self.file_data_source.clone();
    let mut updater = self.updater;

    tokio::spawn(async move {
      //Update store with FILE Datasource
      info!("Starting loading Instance types from File");
      let mut interval = Self::process(&mut updater, &url_datasource);

      loop {
        //Preparing the next tick to update the Store
        let start = Instant::now() + interval;
        let mut time_interval = time::interval_at(start, interval);
        time_interval.tick().await;
        info!("Starting loading Instance types from URL (Not ready yet)");
        interval = Self::process(&mut updater, &url_datasource);
      }
    });
    Ok(())
  }
}
