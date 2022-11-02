//! Entry module for unimarkup-rs.

use logid::capturing::MappedLogId;

use crate::backend;
use crate::config::Config;
use crate::document::Document;
use crate::frontend;
use crate::middleend;

/// Compiles Unimarkup content and returns a [`Document`] representing the given content.
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
            elements: vec![],
            config,
            ..Default::default()
        });
    }

    let mut connection = middleend::setup_ir_connection()?;
    middleend::setup_ir(&connection)?;

    frontend::run(um_content, &mut connection, &mut config)?;
    backend::run(&mut connection, config)
}
