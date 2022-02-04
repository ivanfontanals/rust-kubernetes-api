use anyhow::Result;
use log::info;
use std::process::{Command, Output, Stdio};

use crate::domain::model::GitOpsConfig;
use crate::domain::ports::outgoing::VersionControl;
#[derive(Clone)]
pub struct GitVersionControl {
  config: GitOpsConfig,
  cli_script_path: Option<String>,
}

const GIT_CLI: &str = "./scripts/git_cli.sh";
const CLONE_CMD: &str = "clone";
const PULL_REQUEST_CMD: &str = "pull_request";
const AUTO_COMMIT_CMD: &str = "auto_commit";
const CLEAN_CMD: &str = "clean";

impl GitVersionControl {
  pub fn new(config: GitOpsConfig, cli_script_path: Option<String>) -> Self {
    GitVersionControl { config, cli_script_path }
  }

  fn log_command_output(output: Output) {
    let _ = String::from_utf8(output.stdout).map(|command_log| {
      info!("{:?}", command_log);
    });
  }
}

impl VersionControl for GitVersionControl {
  fn clone_repo(&self, destination_folder: Option<&str>) -> Result<String> {
    let folder = destination_folder.unwrap_or_else(|| self.config.destination_folder.as_str());
    Command::new(self.cli_script_path.as_ref().map_or(GIT_CLI, |s| s))
      .args([CLONE_CMD, self.config.repository_path.as_str(), folder, self.config.branch.as_str()])
      .stdout(Stdio::piped())
      .spawn()
      .and_then(|child| child.wait_with_output())
      .map(|output| {
        Self::log_command_output(output);
        format!("{}/{}", folder, self.config.repository_name)
      })
      .map_err(anyhow::Error::from)
  }

  fn auto_commit(&self, repository_path: String, commit_msg: String) -> Result<()> {
    Command::new(self.cli_script_path.as_ref().map_or(GIT_CLI, |s| s))
      .env("REPO_DIRECTORY", &*repository_path)
      .env("COMMIT_MSG", &*commit_msg)
      .arg(AUTO_COMMIT_CMD)
      .stdout(Stdio::piped())
      .spawn()
      .and_then(|child| child.wait_with_output())
      .map(|output| {
        Self::log_command_output(output);
      })
      .map_err(anyhow::Error::from)
  }

  fn pull_request(&self, repository_path: String, commit_msg: String, title: String, body: String, pr_branch_name: String) -> Result<()> {
    Command::new(self.cli_script_path.as_ref().map_or(GIT_CLI, |s| &s))
      .env("REPO_DIRECTORY", &*repository_path)
      .env("COMMIT_MSG", &*commit_msg)
      .env("PR_TITLE", &*title)
      .env("PR_BODY", &*body)
      .env("PR_BRANCH_NAME", &*pr_branch_name)
      .env("BASE_BRANCH", &*self.config.branch)
      .arg(PULL_REQUEST_CMD)
      .stdout(Stdio::piped())
      .spawn()
      .and_then(|child| child.wait_with_output())
      .map(|output| {
        Self::log_command_output(output);
      })
      .map_err(anyhow::Error::from)
  }

  fn clean(&self, destination_folder: Option<&str>) -> Result<()> {
    let folder_to_delete = destination_folder.unwrap_or_else(|| self.config.destination_folder.as_str());
    Command::new(self.cli_script_path.as_ref().map_or(GIT_CLI, |s| &s))
      .args([CLEAN_CMD, folder_to_delete])
      .stdout(Stdio::piped())
      .spawn()
      .and_then(|child| child.wait_with_output())
      .map(|output| {
        Self::log_command_output(output);
      })
      .map_err(anyhow::Error::from)
  }
}
