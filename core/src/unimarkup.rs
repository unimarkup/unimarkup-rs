//! Entry module for unimarkup-rs.

use logid::capturing::MappedLogId;

use crate::config::Config;
use crate::document::Document;
use crate::frontend;

/// Compiles Unimarkup content, and returns a [`Document`] representing the given content.
///
/// # Arguments
///
/// * `um_content` - String containing Unimarkup elements.
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns a [`MappedLogId`], if error occurs during compilation.
pub fn compile(um_content: &str, mut config: Config) -> Result<Document, MappedLogId> {
    if um_content.is_empty() {
        return Ok(Document {
            blocks: vec![],
            config,
            ..Default::default()
        });
    }

    frontend::run(um_content, &mut config)
}
