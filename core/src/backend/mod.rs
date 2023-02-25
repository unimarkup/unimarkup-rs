//! Backend functionality of [`unimarkup-rs`](crate).
//!
//! Like (re)construction of [`UnimarkupBlocks`] from IR and writing them to files for the given output formats.
//!
//! [`UnimarkupBlocks`]: crate::frontend::UnimarkupBlocks

use crate::config::Config;
use crate::elements::types::{UnimarkupBlocks, UnimarkupFile};
use crate::unimarkup::UnimarkupDocument;

mod renderer;

pub use renderer::*;

use self::error::BackendError;

pub mod error;
pub mod log_id;

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
pub fn run(
    unimarkup_file: UnimarkupFile,
    config: Config,
) -> Result<UnimarkupDocument, BackendError> {
    let blocks: UnimarkupBlocks = unimarkup_file.blocks;

    Ok(UnimarkupDocument {
        elements: blocks,
        config,
    })
}
