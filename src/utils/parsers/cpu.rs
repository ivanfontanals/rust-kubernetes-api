use anyhow::Result;
use regex::{Captures, Regex};
use std::num::ParseFloatError;
use thiserror::Error;

lazy_static! {
  static ref CPU_REGEX: Regex = Regex::new(
    r#"(?x)
      ^(?P<number>\d+)*                           # Number
      [.]?
      (?P<decimal>\d+)*                           # Decimal
      (?P<unit>m)?$       # Unit"#
  )
  .unwrap();
}

#[allow(unused)]
#[derive(Error, PartialEq, Debug)]
pub enum CpuParserError {
  #[error("Input data is invalid")]
  InvalidInput,

  #[error("Error converting string to usize")]
  ConversionError(#[source] ParseFloatError),
}

#[allow(unused)]
pub fn parse_cpu(input: &str) -> Result<usize, CpuParserError> {
  let normalized_input = remove_whitespace(input);

  CPU_REGEX
    .captures(normalized_input.as_ref())
    .ok_or(CpuParserError::InvalidInput)
    .and_then(extract_cpu)
}

fn remove_whitespace(input: &str) -> String {
  input.chars().filter(|c| !c.is_whitespace()).collect()
}

fn extract_cpu(captures: Captures) -> Result<usize, CpuParserError> {
  let cpu_value = format!("{}.{}", extract_number(&captures, "number"), extract_number(&captures, "decimal"));

  match captures.name("unit") {
    Some(_) => convert_cpu_to_usize(cpu_value, 1_f32),
    None => convert_cpu_to_usize(cpu_value, 1000_f32),
  }
}

fn extract_number(captures: &Captures, group_name: &str) -> String {
  captures
    .name(group_name)
    .map_or("0", |regex_match| regex_match.as_str())
    .to_string()
}

fn convert_cpu_to_usize(value: String, unit_scale: f32) -> Result<usize, CpuParserError> {
  value
    .parse()
    .map_err(CpuParserError::ConversionError)
    .map(|value: f32| (value * unit_scale) as usize)
}

#[cfg(test)]
mod tests {
  use crate::utils::parsers::cpu::{parse_cpu, CpuParserError};

  const VALID_TEST_CASES: &[(&str, usize)] = &[
    ("2", 2000),
    ("2.0 ", 2000),
    ("2.3 ", 2300),
    ("500m", 500),
    ("0.1m", 0),
    ("1500m", 1500),
  ];

  const INVALID_TEST_CASES: &[(&str, CpuParserError)] = &[
    ("2x", CpuParserError::InvalidInput),
    ("2 XX", CpuParserError::InvalidInput),
    ("x.5 m", CpuParserError::InvalidInput),
    ("1.2x m", CpuParserError::InvalidInput),
  ];

  #[test]
  fn check_valid_inputs() {
    for (value, expected_output) in VALID_TEST_CASES {
      assert_eq!(parse_cpu(*value).unwrap(), *expected_output)
    }
  }

  #[test]
  fn check_invalid_inputs() {
    for (value, expected_output) in INVALID_TEST_CASES {
      assert_eq!(parse_cpu(*value).unwrap_err(), *expected_output)
    }
  }
}
