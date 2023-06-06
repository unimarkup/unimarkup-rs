//! Entry module for unimarkup-rs.

use crate::document::Document;
use crate::parser;
use unimarkup_commons::config::Config;

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
pub fn compile(um_content: &str, mut config: Config) -> Document {
    if um_content.is_empty() {
        return Document {
            blocks: vec![],
            config,
            ..Default::default()
        };
    }

    parser::parse_unimarkup(um_content, &mut config)
}
