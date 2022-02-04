use std::io::Read;

use crate::domain::model::secrets::SecretRequestDto;
use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait(?Send)]
pub trait Repository<K>: Send
where
  K: Serialize + Send,
{
  fn find_by(&self, name: &str) -> Option<K>;

  fn find_all(&self) -> Option<Vec<K>>;
}

#[async_trait(?Send)]
pub trait DataSource {
  fn name(&self) -> &str;
  fn reader<'a>(&'a self) -> Result<Box<dyn Read + 'a>>;
}

pub trait ReadStore<T>
where
  T: Serialize,
{
  fn get<S: AsRef<str>>(&self, name: S) -> Result<T>;
  fn list(&self) -> Result<Vec<T>>;
}

pub trait WriteStore<T>
where
  T: Serialize,
{
  fn update(&mut self, instance_types: Vec<T>) -> Result<()>;
}

#[async_trait(?Send)]
pub trait SealedSecretClient {
  fn save(&self, request: &SecretRequestDto, destination: Option<String>) -> Result<()>;

  /// Render a Selead Secret. This methis will not change or create nothing.
  ///
  /// # Arguments
  ///
  /// * `request` - The  secret request
  ///
  fn render(&self, request: &SecretRequestDto) -> Result<String>;
}

#[async_trait(?Send)]
pub trait VersionControl {
  /// Clones a repo and returns the destination folder where the project has been cloned.
  ///
  /// # Arguments
  ///
  /// * `destination_folder` - Optional. Use this folder in case you want to override the default one
  ///

  fn clone_repo(&self, destination_folder: Option<&str>) -> Result<String>;

  /// Creates a commit and a push to the target remote repository.
  ///
  /// # Arguments
  ///
  /// * `repository_path` - The local folder where the repository has been cloned
  /// * `message` - The commit message
  ///
  fn auto_commit(&self, repository_path: String, message: String) -> Result<()>;

  /// Creates a Pull Request from the local repository.
  ///
  /// # Arguments
  ///
  /// * `repository_path` - The local folder where the repository has been cloned
  /// * `pr_branch_name` - The branch that will be created to create the PR
  /// * `commit_msg` - The commit message that will be used in the pr_branch_name
  /// * `title` - The PR title
  /// * `body` - The description message for the PR
  ///
  fn pull_request(&self, repository_path: String, commit_msg: String, title: String, body: String, pr_branch_name: String) -> Result<()>;

  /// Cleans the local folder where the repository has been cloned.
  /// By default, the clean method will delete the default destintation folder used in the creation
  ///
  /// # Arguments
  ///
  /// * `destination_folder` - Optional. In case you have used another destintation folder, you can use this attribute. This can be usefull when several users make actions at the same time.
  ///
  fn clean(&self, destination_folder: Option<&str>) -> Result<()>;
}
