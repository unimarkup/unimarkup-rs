//! Backend functionality of [`unimarkup-rs`](crate).
//!
//! Like (re)construction of [`UnimarkupBlocks`] from IR and writing them to files for the given output formats.
//!
//! [`UnimarkupBlocks`]: crate::frontend::UnimarkupBlocks

use crate::{
    config::{Config, OutputFormat},
    um_error::UmError,
};
use log::info;
use rusqlite::Connection;

mod backend_error;
mod loader;
mod renderer;
pub(crate) mod inline_formatting;

pub use backend_error::BackendError;
pub use loader::ParseFromIr;
pub use renderer::*;

/// Abstract type for elements that implement the [`Render`] trait
pub type RenderBlock = Box<dyn Render>;

/// This is the entry function for the [`backend`](crate::backend) module. It fetches
/// [`UnimarkupBlocks`] from IR, renders them to the wanted output format and writes the resulting output.
///
/// This function will return an [`UmError`] if
///
/// - connection to the IR fails
/// - reconstructing of [`UnimarkupBlocks`] fails
/// - error occurs when writing to the output file
///
/// [`UnimarkupBlocks`]: crate::frontend::UnimarkupBlocks
pub fn run(connection: &mut Connection, config: &Config) -> Result<(), UmError> {
    let blocks: Vec<RenderBlock> = loader::get_blocks_from_ir(connection)?;

    let out_path = {
        if let Some(ref out_file) = config.out_file {
            out_file.clone()
        } else {
            let mut in_file = config.um_file.clone();
            in_file.set_extension("");

            in_file
        }
    };

    if let Some(ref output_formats) = config.out_formats {
        if output_formats.contains(&OutputFormat::Html) {
            let html = renderer::render_html(&blocks)?;

            let mut out_path_html = out_path;
            out_path_html.set_extension("html");

            let out_path = out_path_html.to_str().expect("Validation done in config");

            info!("Writing to {}", out_path);

            std::fs::write(&out_path_html, &html).map_err(|err| {
                BackendError::new(format!(
                    "Could not write to file '{}'.\nReason: {}",
                    out_path, err
                ))
            })?;
        }
    }

    Ok(())
}
