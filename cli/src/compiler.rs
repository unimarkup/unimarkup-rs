//! Entry module for unimarkup-rs.

use std::io::Write;
use std::{
    fs,
    path::{Path, PathBuf},
};

use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use logid::{log, logging::event_entry::AddonKind, pipe};
use tempfile::Builder;

use unimarkup_core::commons::config::output::OutputFormatKind::Pdf;
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
                &um.render_html()
                    .map_err(|_| GeneralError::Render)?
                    .to_string(),
                &out_path,
                format.extension(),
            )?,
            OutputFormatKind::Pdf => create_pdf_data(
                &um.render_html()
                    .map_err(|_| GeneralError::Render)?
                    .to_string(),
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

fn create_pdf_data(html: &str, out_path: impl AsRef<Path>) -> Result<(), GeneralError> {
    let mut temp_html_file = Builder::new()
        .suffix(".html")
        .tempfile()
        .map_err(|_| GeneralError::FileWrite)?;

    temp_html_file
        .write_all(html.as_bytes())
        .map_err(|_| GeneralError::FileWrite)?;
    let temp_file_url = format!(
        "file://{}",
        temp_html_file
            .as_ref()
            .as_os_str()
            .to_str()
            .ok_or(GeneralError::FileWrite)?
    );

    let browser = Browser::new(LaunchOptions::default()).expect("Error");
    let pdf_bytes = browser
        .new_tab()
        .map_err(|_| GeneralError::Render)?
        .navigate_to(temp_file_url.as_str())
        .map_err(|_| GeneralError::FileRead)?
        .wait_until_navigated()
        .map_err(|_| GeneralError::FileRead)?
        .print_to_pdf(Some(PrintToPdfOptions::default()))
        .map_err(|_| GeneralError::Render)?;

    let mut full_out_path: PathBuf = out_path.as_ref().into();
    full_out_path.set_extension(Pdf.extension());
    std::fs::write(full_out_path, pdf_bytes).map_err(|_| GeneralError::Render)?;

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
