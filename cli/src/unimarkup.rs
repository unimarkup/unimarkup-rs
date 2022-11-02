//! Entry module for unimarkup-rs.

use std::fs;

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use unimarkup_core::config::{Config, OutputFormat};

use crate::log_id::{GeneralErrLogId, GeneralInfLogId, CLI_LOG_ID_MAP};

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns a [`MappedLogId`] if error occurs during compilation.
pub fn compile(config: Config) -> Result<(), MappedLogId> {
    let source = fs::read_to_string(&config.um_file).map_err(|err| {
        (GeneralErrLogId::FailedReadingFile as LogId)
            .set_event_with(
                &CLI_LOG_ID_MAP,
                &format!("Could not read file: '{:?}'", &config.um_file),
                file!(),
                line!(),
            )
            .add_info(&format!("Cause: {}", err))
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

            (GeneralInfLogId::WritingToFile as LogId).set_event_with(
                &CLI_LOG_ID_MAP,
                &format!("Writing to file: {:?}", out_path_html),
                file!(),
                line!(),
            );

            std::fs::write(&out_path_html, &html.body).map_err(|err| {
                (GeneralErrLogId::FailedWritingFile as LogId)
                    .set_event_with(
                        &CLI_LOG_ID_MAP,
                        &format!("Could not write to file: {:?}", out_path_html),
                        file!(),
                        line!(),
                    )
                    .add_info(&format!("Cause: {}", err))
            })?;
        }
    }

    Ok(())
}
