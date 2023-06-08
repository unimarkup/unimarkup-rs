//! Entry module for unimarkup-rs.

use std::{
    fs,
    path::{Path, PathBuf},
};

use logid::{log, logging::event_entry::AddonKind, pipe};
use unimarkup_commons::config::{output::OutputFormat, Config};
use unimarkup_core::document::Document;

use crate::log_id::{GeneralError, GeneralInfo};

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns a [`GeneralError`] if error occurs during compilation.
pub fn compile(config: Config) -> Result<(), GeneralError> {
    let source = fs::read_to_string(&config.input).map_err(|error| {
        pipe!(
            GeneralError::FileRead,
            &format!("Could not read file: '{:?}'", &config.input),
            add: AddonKind::Info(format!("Cause: {}", error))
        )
    })?;

    let out_path = {
        if let Some(ref out_file) = config.preamble.output.file {
            out_file.clone()
        } else {
            let mut in_file = config.input.clone();
            in_file.set_extension("");

            in_file
        }
    };

    let document = unimarkup_core::unimarkup::compile(&source, config);

    for format in document.output_formats() {
        match format {
            OutputFormat::Html => write_html(&document, &out_path)?,
        }
    }

    Ok(())
}

fn write_html(document: &Document, out_path: impl AsRef<Path>) -> Result<(), GeneralError> {
    let html = document.html();

    let mut out_path_html: PathBuf = out_path.as_ref().into();
    out_path_html.set_extension("html");

    log!(
        GeneralInfo::WritingToFile,
        &format!("Writing to file: {:?}", out_path_html),
    );

    std::fs::write(&out_path_html, html.body).map_err(|error| {
        pipe!(
            GeneralError::FileWrite,
            &format!("Could not write to file: {:?}", out_path_html),
            add: AddonKind::Info(format!("Cause: {}", error))
        )
    })
}
