//! Entry module for unimarkup-rs.

use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use logid::{log, logging::event_entry::AddonKind, pipe};

use unimarkup_core::{
    commons::config::{output::OutputFormatKind, Config},
    Unimarkup,
};

use crate::log_id::{GeneralError, GeneralInfo};

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to *unimarkup*.
///
/// # Errors
///
/// Returns a [`GeneralError`] if error occurs during compilation.
pub fn compile(config: Config) -> Result<(), GeneralError> {
    let source = fs::read_to_string(&config.input).map_err(|error| {
        pipe!(
            GeneralError::FileRead,
            format!("Could not read file: '{:?}'", &config.input),
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
                &um.render_html(false)
                    .map_err(|_| GeneralError::Render)?
                    .to_string(),
                &out_path,
                format.extension(),
            )?,
            OutputFormatKind::Pdf => write_raw_file(
                um.render_pdf().map_err(|_| GeneralError::Render)?,
                &out_path,
                format.extension(),
            )?,
            OutputFormatKind::Umi => write_file(
                &um.render_umi()
                    .map_err(|_| GeneralError::Render)?
                    .create_workbook()
                    .to_string(),
                &out_path,
                OutputFormatKind::Umi.extension(),
            )?,
        }
    }

    Ok(())
}

fn write_raw_file(
    content: Vec<u8>,
    out_path: impl AsRef<Path>,
    extension: &str,
) -> Result<(), GeneralError> {
    let mut full_out_path: PathBuf = out_path.as_ref().into();
    full_out_path.set_extension(extension);

    log!(
        GeneralInfo::WritingToFile,
        format!("Writing to file: {:?}", full_out_path),
    );

    std::fs::write(&full_out_path, content).map_err(|error| {
        pipe!(
            GeneralError::FileWrite,
            format!("Could not write to file: {:?}", full_out_path),
            add: AddonKind::Info(format!("Cause: {}", error))
        )
    })
}

fn write_file(
    content: &str,
    out_path: impl AsRef<Path>,
    extension: &str,
) -> Result<(), GeneralError> {
    write_raw_file(content.as_bytes().to_vec(), out_path, extension)
}
