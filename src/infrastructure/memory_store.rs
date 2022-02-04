use anyhow::anyhow;
use anyhow::{Error, Result};
use kv::{Bincode, Bucket, Codec};
use serde::Serialize;
use std::{collections::HashSet, path::Path};

use crate::domain::ports::incoming::WithName;
use crate::domain::ports::outgoing::{ReadStore, WriteStore};

#[derive(Clone)]
pub struct InMemoryStore<'a, T>
where
  T: Serialize + serde::de::DeserializeOwned + WithName,
{
  pub(self) bucket: Bucket<'a, String, Bincode<T>>,
}

impl<'a, T> InMemoryStore<'a, T>
where
  T: Serialize + serde::de::DeserializeOwned + WithName,
{
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
    let config = kv::Config::new(path.as_ref());
    let store = kv::Store::new(config)?;
    let bucket = store.bucket::<String, Bincode<T>>(None)?;
    Ok(Self { bucket })
  }
}

impl<'a, T> ReadStore<T> for InMemoryStore<'a, T>
where
  T: Serialize + serde::de::DeserializeOwned + WithName,
{
  fn get<S: AsRef<str>>(&self, name: S) -> Result<T, Error> {
    if let Ok(Some(bincode)) = self.bucket.get(name.as_ref()) {
      Ok(bincode.into_inner())
    } else {
      Err(anyhow!("Key not found: {}", name.as_ref()))
    }
  }

  fn list(&self) -> Result<Vec<T>> {
    Ok(
      self
        .bucket
        .iter()
        .filter_map(|maybe_item| maybe_item.and_then(|item| item.value::<Bincode<T>>().map(|x| x.0)).ok())
        .collect(),
    )
  }
}

impl<'a, T> WriteStore<T> for InMemoryStore<'a, T>
where
  T: Serialize + serde::de::DeserializeOwned + WithName,
{
  fn update(&mut self, items: Vec<T>) -> Result<()> {
    let updated_keys: HashSet<String> = items.iter().map(|instance_type| instance_type.name()).collect();

    let existing_keys: HashSet<String> = self
      .bucket
      .iter()
      .filter_map(|maybe_item| maybe_item.and_then(|item| item.key::<String>()).ok())
      .collect();

    let mut batch = kv::Batch::new();

    for item in items {
      batch.set(item.name().clone(), Bincode(item))?;
    }

    let keys_to_delete = existing_keys.difference(&updated_keys);
    for key in keys_to_delete {
      batch.remove(key)?;
    }

    self.bucket.batch(batch)?;
    Ok(())
  }
}
