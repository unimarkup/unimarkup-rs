//! Backend functionality of [`unimarkup-rs`](crate).
//!
//! Like (re)construction of [`UnimarkupBlocks`] from IR, and writing them to files for the given output formats.

use crate::{config::Config, document::Document, elements::UnimarkupBlocks};
use logid::capturing::MappedLogId;
use rusqlite::Connection;

mod loader;

pub use loader::ParseFromIr;

pub mod log_id;

/// This is the entry function for the [`backend`](crate::backend) module. It fetches
/// [`UnimarkupBlocks`] from IR, renders them to the wanted output format and writes the resulting output.
///
/// This function will return a [`MappedLogId`] if
///
/// - connection to the IR fails
/// - reconstructing of [`UnimarkupBlocks`] fails
/// - error occurs when writing to the output file
pub fn run(connection: &mut Connection, config: Config) -> Result<Document, MappedLogId> {
    let blocks: UnimarkupBlocks = loader::get_blocks_from_ir(connection)?;

    Ok(Document {
        elements: blocks,
        config,
        ..Default::default()
    })
}
