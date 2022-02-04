use std::env;
use thiserror::Error;

use crate::domain::model::GitOpsConfig;

type Result<T> = core::result::Result<T, EnvConfigError>;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EnvConfigError {
  #[error("Missing environment variable: {0}")]
  MissingEnvVar(String),
}

/// Helper providing application configuration injected through the environment variables
pub struct EnvConfig;

impl EnvConfig {
  #[allow(unused)]
  pub fn app_name() -> Result<String> {
    Self::var("APP_NAME")
  }

  #[allow(unused)]
  pub fn app_version() -> Result<String> {
    Self::var("APP_VERSION")
  }

  pub fn namespace() -> Result<String> {
    Self::var("NAMESPACE")
  }

  pub fn gitops_config() -> Result<GitOpsConfig> {
    let organization = Self::var("GITPOS_ORGANIZATION")?;
    let repository_name = Self::var("GITPOS_REPO")?;
    let destination_folder = Self::var("GITPOS_DESTINATION_FOLDER")?;
    let branch = Self::var("GITPOS_BRANCH")?;
    Ok(GitOpsConfig {
      repository_name: repository_name.clone(),
      organization: organization.clone(),
      branch,
      destination_folder,
      repository_path: format!("{}/{}", organization, repository_name),
    })
  }

  pub fn stores_path() -> Result<String> {
    Self::var("STORES_PATH").or_else(|_| Ok(env::temp_dir().join("stores").to_string_lossy().into_owned()))
  }

  pub fn instance_types_file_source() -> Result<String> {
    Self::var("INSTANCE_TYPES_FILE_SOURCE").or_else(|err| {
      env::current_dir()
        .map(|current_dir| current_dir.join("pricing-list.json").to_string_lossy().into_owned())
        .map_err(|_| err)
    })
  }

  fn var(name: &str) -> Result<String> {
    let non_empty_string = |value: &String| !value.is_empty();
    let missing_env_var_error = || EnvConfigError::MissingEnvVar(name.to_string());

    env::var(name).ok().filter(non_empty_string).ok_or_else(missing_env_var_error)
  }
}
