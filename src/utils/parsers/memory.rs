use anyhow::Result;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::num::ParseFloatError;
use thiserror::Error;

lazy_static! {
  static ref UNIT_SCALE_MAPPING: HashMap<&'static str, f32> = {
    let mut unit_scale = HashMap::new();
    unit_scale.insert("k", 1000.0);
    unit_scale.insert("Ki", 1024.0);
    unit_scale.insert("KiB", 1024.0);
    unit_scale.insert("M", 1000000.0);
    unit_scale.insert("Mi", 1048576.0);
    unit_scale.insert("MiB", 1048576.0);
    unit_scale.insert("G", 1000000000.0);
    unit_scale.insert("Gi", 1073741824.0);
    unit_scale.insert("GiB", 1073741824.0);
    unit_scale
  };
  static ref MEMORY_REGEX: Regex = Regex::new(
    r#"(?x)
      ^(?P<number>\d+)*                           # Number
      [.]?
      (?P<decimal>\d+)*                           # Decimal
      (?P<unit>KiB|Ki|k|MiB|Mi|M|GiB|Gi|G)$       # Unit"#
  )
  .unwrap();
}

#[derive(Error, PartialEq, Debug)]
pub enum MemoryParserError {
  #[error("Input data is invalid")]
  InvalidInput,

  #[error("Error converting string to usize")]
  ConversionError(#[source] ParseFloatError),
}

#[allow(unused)]
pub fn parse_memory_in_bytes<S: AsRef<str>>(input: S) -> Result<usize, MemoryParserError> {
  let normalized_input = remove_whitespace(input.as_ref());

  MEMORY_REGEX
    .captures(normalized_input.as_ref())
    .ok_or(MemoryParserError::InvalidInput)
    .and_then(extract_memory_in_bytes)
}

fn remove_whitespace(input: &str) -> String {
  input.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn extract_memory_in_bytes(captures: Captures) -> Result<usize, MemoryParserError> {
  let memory_value = format!("{}.{}", extract_number(&captures, "number"), extract_number(&captures, "decimal"));

  captures
    .name("unit")
    .and_then(|unit_match| UNIT_SCALE_MAPPING.get(unit_match.as_str()).cloned())
    .ok_or(MemoryParserError::InvalidInput)
    .and_then(|unit_scale| convert_memory_to_usize(memory_value, unit_scale))
}

fn extract_number(captures: &Captures, group_name: &str) -> String {
  captures
    .name(group_name)
    .map_or("0", |regex_match| regex_match.as_str())
    .to_string()
}

fn convert_memory_to_usize(value: String, unit_scale: f32) -> Result<usize, MemoryParserError> {
  value
    .parse()
    .map_err(MemoryParserError::ConversionError)
    .map(|value: f32| (value * unit_scale).ceil() as usize)
}

#[cfg(test)]
mod tests {
  use crate::utils::parsers::memory::{parse_memory_in_bytes, MemoryParserError};

  const VALID_TEST_CASES: &[(&str, usize)] = &[
    ("2 KiB", 2048),
    ("2. Ki", 2048),
    (".5 KiB", 512),
    ("1.5 KiB", 1536),
    ("0.5 k", 500),
    ("2 MiB", 2097152),
    ("2. Mi", 2097152),
    ("2.0 Mi", 2097152),
    ("0.5 M", 500000),
    ("2 GiB", 2147483648),
    ("2. Gi", 2147483648),
    ("2.0 Gi", 2147483648),
    ("0.5 G", 500000000),
  ];

  const INVALID_TEST_CASES: &[(&str, MemoryParserError)] = &[
    ("2x KiB", MemoryParserError::InvalidInput),
    ("2 XX", MemoryParserError::InvalidInput),
    ("x.5 KiB", MemoryParserError::InvalidInput),
    ("1.2x M", MemoryParserError::InvalidInput),
    ("1.2 Mx", MemoryParserError::InvalidInput),
  ];

  #[test]
  fn check_valid_inputs() {
    for (value, expected_output) in VALID_TEST_CASES {
      assert_eq!(parse_memory_in_bytes(*value).unwrap(), *expected_output)
    }
  }

  #[test]
  fn check_invalid_inputs() {
    for (value, expected_output) in INVALID_TEST_CASES {
      assert_eq!(parse_memory_in_bytes(*value).unwrap_err(), *expected_output)
    }
  }
}
