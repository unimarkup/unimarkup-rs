//! Entry module for unimarkup-rs.

use std::{
    fs,
    path::{Path, PathBuf},
};

use logid::{log, logging::event_entry::AddonKind, pipe};
use unimarkup_commons::config::{output::OutputFormatKind, Config};
use unimarkup_core::Unimarkup;

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
        if let Some(ref out_file) = config.output.file {
            out_file.clone()
        } else {
            let mut in_file = config.input.clone();
            in_file.set_extension("");

            in_file
        }
    };

    let um = Unimarkup::parse(&source, config);
    for format in um.get_formats() {
        match format {
            OutputFormatKind::Html => write_file(
                &um.render_html()
                    .map_err(|_| GeneralError::Render)?
                    .to_string(),
                &out_path,
                OutputFormatKind::Html.extension(),
            )?,
        }
    }

    Ok(())
}

fn write_file(
    content: &str,
    out_path: impl AsRef<Path>,
    extension: &str,
) -> Result<(), GeneralError> {
    let mut full_out_path: PathBuf = out_path.as_ref().into();
    full_out_path.set_extension(extension);

    log!(
        GeneralInfo::WritingToFile,
        &format!("Writing to file: {:?}", full_out_path),
    );

    std::fs::write(&full_out_path, content).map_err(|error| {
        pipe!(
            GeneralError::FileWrite,
            &format!("Could not write to file: {:?}", full_out_path),
            add: AddonKind::Info(format!("Cause: {}", error))
        )
    })
}
