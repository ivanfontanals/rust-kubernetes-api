#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitOpsConfig {
  pub repository_name: String,
  pub organization: String,
  pub branch: String,
  pub destination_folder: String,
  pub repository_path: String,
}
