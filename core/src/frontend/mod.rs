//! Frontend functionality of [`unimarkup-rs`](crate).
//!
//! i.e. parsing of unimarkup-rs files, generating corresponding
//! ['Blocks'] and sending them to the IR.

use logid::capturing::MappedLogId;

use crate::document::Document;
use unimarkup_commons::config::Config;

pub mod log_id;
pub mod parser;

/// `frontend::run` is the entry function of the [`frontend`] module.
/// It parses a Unimarkup file, and returns a Unimarkup [`Document`].
///
/// # Errors
///
/// This function will return an error if the given Unimarkup file contains invalid syntax.
///
/// [`frontend`]: crate::frontend
/// [`Document`]: crate::document
pub fn run(um_content: &str, config: &mut Config) -> Result<Document, MappedLogId> {
    let unimarkup = crate::parser::parse_unimarkup(um_content, config)?;

    Ok(unimarkup)
}
