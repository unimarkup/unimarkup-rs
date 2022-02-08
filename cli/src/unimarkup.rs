//! Entry module for unimarkup-rs.

use std::fs;

use log::info;
use unimarkup_core::{config::{Config, OutputFormat}, log_id::{LogId, SetLog}};

use crate::{error::CliError, log_id::GeneralErrLogId};

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns an [`CliError`], if error occurs during compilation.
pub fn compile(config: Config) -> Result<(), CliError> {
    let source = fs::read_to_string(&config.um_file).map_err(|err| 
        CliError::General(
            (GeneralErrLogId::FailedReadingFile as LogId)
            .set_log(&format!("Could not read file: '{:?}'", &config.um_file), file!(), line!())
            .add_to_log(&format!("Cause: {}", err))
        )
    )?;

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

            std::fs::write(&out_path_html, &html.body()).map_err(|err| 
                CliError::General(
                    (GeneralErrLogId::FailedReadingFile as LogId)
                    .set_log(&format!("Could not write to file: '{:?}'", out_path), file!(), line!())
                    .add_to_log(&format!("Cause: {}", err))
                )
            )?;
        }
    }

    Ok(())
}
