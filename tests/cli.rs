use std::path::PathBuf;

use clap::Parser;
use shlex::Shlex;
use unimarkup_rs::{config::{Config, OutputFormat}, um_error::UmError};

fn get_args(options: &str, um_file: &str) -> Vec<String> {
  let arg_line = format!("unimarkup {} {}", options, um_file);
  Shlex::new(&arg_line).collect()
}

#[test]
fn only_required_arguments_to_struct() -> Result<(), UmError> {
  let um_filename = "file.um";
  let cfg: Config = Config::parse_from(get_args("", um_filename));

  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename not set correctly");

  Ok(())
}

#[test]
fn single_output_format() -> Result<(), UmError> {
  let um_filename = "file.um";
  let options = "--output-formats=html";

  let cfg: Config = Config::parse_from(get_args(options, um_filename));

  assert_eq!(cfg.out_formats.unwrap()[0], OutputFormat::Html, "Unimarkup html output format not set correctly");
  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename not set correctly");

  Ok(())
}

#[test]
fn multiple_output_formats() -> Result<(), UmError> {
  let um_filename = "file.um";
  let options = "--output-formats=html,pdf";

  let cfg: Config = Config::parse_from(get_args(options, um_filename));
  let formats = cfg.out_formats.unwrap();

  assert_eq!(formats[0], OutputFormat::Html, "Unimarkup html output format not set correctly");
  assert_eq!(formats[1], OutputFormat::Pdf, "Unimarkup html output format not set correctly");
  assert!(formats.len() == 2, "Too many Unimarkup output formats set");

  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename not set correctly");

  Ok(())
}

#[test]
fn theme_option_set() -> Result<(), UmError> {
  let um_filename = "file.um";
  let theme = "not_existing_theme.um";

  let options = format!("--theme={}", theme);
  let args = get_args(&options, um_filename);

  let cfg: Config = Config::parse_from(args);

  assert_eq!(cfg.theme.unwrap(), PathBuf::from(theme), "Theme file set correctly");

  Ok(())
}

#[test]
fn bad_theme_path() -> Result<(), UmError> {
  let um_filename = "file.um";
  let theme = "not_existing_theme.um";

  let options = format!("--theme={}", theme);
  let args = get_args(&options, um_filename);

  let cfg: Config = Config::parse_from(args);

  assert!(!cfg.theme.unwrap().exists(), "Theme file should not exist");

  Ok(())
}
