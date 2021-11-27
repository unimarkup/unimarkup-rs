use std::path::PathBuf;

use clap::Parser;
use shlex::Shlex;
use unimarkup_rs::{config::{Config, OutputFormat}, um_error::UmError};


#[test]
fn only_required_arguments_to_struct() -> Result<(), UmError> {
  let um_filename = "file.um";
  let cfg: Config = Config::parse_from(vec!["unimarkup", um_filename]);

  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename set correctly");

  Ok(())
}

#[test]
fn single_output_config() -> Result<(), UmError> {
  let um_filename = "file.um";
  let output = "-O --output-formats=html,pdf";

  let cfg: Config = Config::parse_from(vec!["unimarkup", output, um_filename]);
  let cfg_outputs = cfg.outputs.unwrap();

  assert_eq!(cfg_outputs[0].out_formats[0], OutputFormat::Html, "Unimarkup html output format set correctly");
  assert_eq!(cfg_outputs[0].out_formats[1], OutputFormat::Pdf, "Unimarkup html output format set correctly");
  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename set correctly");

  Ok(())
}

#[test]
fn multiple_output_configs() -> Result<(), UmError> {
  let um_filename = "file.um";
  let output = "-O=\"--output-formats=html,pdf\"";
  let theme = "test_theme.um";
  let output_2 = format!("-O=\"--output-formats=intermediate,revealjs --theme={}\"", theme);

  let arg_line = format!("unimarkup {} {} {}", output, output_2, um_filename);
  let args = Shlex::new(&arg_line);

  let cfg: Config = Config::parse_from(args);
  let cfg_outputs = cfg.outputs.unwrap();

  assert_eq!(cfg_outputs[0].out_formats[0], OutputFormat::Html, "Unimarkup html output format set correctly");
  assert_eq!(cfg_outputs[0].out_formats[1], OutputFormat::Pdf, "Unimarkup html output format set correctly");
  assert!(cfg_outputs[0].out_formats.len() == 2, "Too many Unimarkup output formats set");

  assert_eq!(cfg_outputs[1].out_formats[0], OutputFormat::Intermediate, "Unimarkup intermediate output format set correctly");
  assert_eq!(cfg_outputs[1].out_formats[1], OutputFormat::RevealJs, "Unimarkup revealJs output format set correctly");
  assert_eq!(cfg_outputs[1].theme, Some(PathBuf::from(theme)), "Unimarkup theme for second output config set correctly");
  assert!(cfg_outputs[1].out_formats.len() == 2, "Too many Unimarkup output formats set");

  assert_eq!(cfg.um_file.to_str().unwrap(), um_filename, "Unimarkup filename set correctly");

  Ok(())
}
