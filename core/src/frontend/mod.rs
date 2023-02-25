//! Frontend functionality of [`unimarkup-rs`](crate).
//!
//! i.e. parsing of unimarkup-rs files, generating corresponding
//! ['UnimarkupBlocks'] and sending them to the IR.

use crate::config::Config;
use crate::elements::types::UnimarkupFile;

use self::error::FrontendError;

pub mod error;
pub mod log_id;
pub mod parser;
pub mod preamble;

/// `frontend::run` is the entry function of the [`frontend`] module.
/// It parses a Unimarkup file and sends the data to the IR.
///
/// # Errors
///
/// This function will return an error if the given Unimarkup file contains invalid syntax,
/// or if communication with IR fails.
///
/// [`frontend`]: crate::frontend
pub fn run(um_content: &str, config: &mut Config) -> Result<UnimarkupFile, FrontendError> {
    let unimarkup = parser::parse_unimarkup(um_content, config)?;

    Ok(unimarkup)
}
