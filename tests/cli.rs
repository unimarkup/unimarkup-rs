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
fn single_output_config() -> Result<(), UmError> {
  let um_filename = "file.um";
  let out_name = "out";
  let output = format!("-O=\"{} --output-formats=html,pdf\"", out_name);

  let cfg: Config = Config::parse_from(get_args(&output, um_filename));
  let cfg_outputs = cfg.outputs.unwrap();

  assert_eq!(cfg_outputs[0].1.out_formats[0], OutputFormat::Html, "Unimarkup html output format not set correctly");
  assert_eq!(cfg_outputs[0].1.out_formats[1], OutputFormat::Pdf, "Unimarkup html output format not set correctly");
  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename not set correctly");
  assert_eq!(cfg_outputs[0].0, out_name, "Output config name not set correctly");

  Ok(())
}

#[test]
fn multiple_output_configs() -> Result<(), UmError> {
  let um_filename = "file.um";
  let out_1_name = "out_1";
  let output_1 = format!("-O=\"{} --output-formats=html,pdf\"", out_1_name);
  let theme = "test_theme.um";
  let out_2_name = "out_2";
  let output_2 = format!("-O=\"{} --output-formats=intermediate,revealjs --theme={}\"", out_2_name, theme);

  let options = format!("{} {}", output_1, output_2);
  let args = get_args(&options, um_filename);

  let cfg: Config = Config::parse_from(args);
  let cfg_outputs = cfg.outputs.unwrap();

  assert_eq!(cfg_outputs[0].1.out_formats[0], OutputFormat::Html, "Unimarkup html output format not set correctly");
  assert_eq!(cfg_outputs[0].1.out_formats[1], OutputFormat::Pdf, "Unimarkup html output format not set correctly");
  assert!(cfg_outputs[0].1.out_formats.len() == 2, "Too many Unimarkup output formats set");
  assert_eq!(cfg_outputs[0].0, out_1_name, "Output_1 config name not set correctly");

  assert_eq!(cfg_outputs[1].1.out_formats[0], OutputFormat::Intermediate, "Unimarkup intermediate output format not set correctly");
  assert_eq!(cfg_outputs[1].1.out_formats[1], OutputFormat::RevealJs, "Unimarkup revealJs output format not set correctly");
  assert_eq!(cfg_outputs[1].1.theme, Some(PathBuf::from(theme)), "Unimarkup theme for second output config not set correctly");
  assert!(cfg_outputs[1].1.out_formats.len() == 2, "Too many Unimarkup output formats set");
  assert_eq!(cfg_outputs[1].0, out_2_name, "Output_2 config name not set correctly");

  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename not set correctly");

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
