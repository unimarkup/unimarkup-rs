//! Entry module for unimarkup-rs.

use std::fs;

use log::info;
use unimarkup_core::backend::BackendError;
use unimarkup_core::config::{Config, OutputFormat};
use unimarkup_core::error::UmError;

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns an [`UmError`], if error occurs during compilation.
pub fn compile(config: Config) -> Result<(), UmError> {
    let source = fs::read_to_string(&config.um_file).map_err(|err| UmError::General {
        msg: String::from("Could not read file."),
        error: Box::new(err),
    })?;

    let out_path = {
        if let Some(ref out_file) = config.out_file {
            out_file.clone()
        } else {
            let mut in_file = config.um_file.clone();
            in_file.set_extension("");

            in_file
        }
    };

    let document = unimarkup_core::unimarkup::compile(&source, config)?;

    if let Some(output_formats) = document.output_formats() {
        if output_formats.contains(&OutputFormat::Html) {
            let html = document.html();

            let mut out_path_html = out_path;
            out_path_html.set_extension("html");

            let out_path = out_path_html.to_str().expect("Validation done in config");

            info!("Writing to {}", out_path);

            std::fs::write(&out_path_html, &html.body()).map_err(|err| {
                BackendError::new(format!(
                    "Could not write to file '{}'.\nReason: {}",
                    out_path, err
                ))
            })?;
        }
    }

    Ok(())
}
