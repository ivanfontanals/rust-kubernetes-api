use anyhow::{anyhow, Result};
use base64::decode;
use std::{
  fs::write,
  process::{Command, Output, Stdio},
};

use crate::domain::model::secrets::SecretRequestDto;
use crate::domain::ports::outgoing::SealedSecretClient;

#[derive(Clone)]
pub struct KubesealClient {
  cli_script_path: Option<String>,
}

const KUBESEAL_CLI: &str = "./scripts/kubeseal_cli.sh";
const RENDER_CMD: &str = "render";

impl KubesealClient {
  pub fn new(cli_script_path: Option<String>) -> Self {
    KubesealClient { cli_script_path }
  }

  fn extract_output(output: Output) -> String {
    match String::from_utf8(output.stdout) {
      Ok(output) => output,
      Err(error) => error.to_string(),
    }
  }

  fn extract_output_result(output: Output) -> Result<String> {
    let std_output = Self::extract_output(output.clone());
    match output.status.success() {
      true => Ok(std_output),
      false => Err(anyhow!("Error in kubeseal script: {}", std_output)),
    }
  }
}

impl SealedSecretClient for KubesealClient {
  fn save(&self, request: &SecretRequestDto, _destination: Option<String>) -> Result<()> {
    let rendered_yaml = self.render(request)?;
    let default_destination = "why_is_destination_an_option?".to_string();
    write(_destination.unwrap_or(default_destination), rendered_yaml).map_err(|err| err.into())
  }

  fn render(&self, request: &SecretRequestDto) -> Result<String> {
    let literals_str = request
      .literals
      .clone()
      .into_iter()
      .map(|(key, value)| format!(" --from-literal={}={}", key, value))
      .fold(String::new(), |output, literal| format!("{}{}", output, literal));

    Command::new(self.cli_script_path.as_ref().map_or(KUBESEAL_CLI, |s| s))
      .env("LITERALS", &*literals_str)
      .arg(RENDER_CMD)
      .arg(&request.name)
      .stdout(Stdio::piped())
      .spawn()
      .and_then(|child| child.wait_with_output())
      .map_err(|error| anyhow::Error::from(error))
      .and_then(|output| Self::extract_output_result(output))
  }
}
// kubectl create secret generic test-db-secret --dry-run --from-literal=username=testuser --from-literal=password=iluvtests -o json | kubeseal -o yaml
