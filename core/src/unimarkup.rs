//! Entry module for unimarkup-rs.

use crate::backend;
use crate::config::Config;
use crate::elements::types::UnimarkupBlocks;
use crate::error::UmError;
use crate::frontend;
use crate::middleend;

/// Struct representing a Unimarkup document that can be rendered to supported output formats.
pub struct UnimarkupDocument {
    pub(crate) elements: Vec<UnimarkupBlocks>,
}

/// Compiles Unimarkup content and returns a [`UnimarkupDocument`] representing the given content.
///
/// # Arguments
///
/// * `um_content` - String containing Unimarkup elements.
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns a [`UmError`], if error occurs during compilation.
pub fn compile(um_content: &str, mut config: Config) -> Result<UnimarkupDocument, UmError> {
    let mut connection = middleend::setup_ir_connection()?;
    middleend::setup_ir(&connection)?;

    frontend::run(um_content, &mut connection, &mut config)?;
    Ok(backend::run(&mut connection, &config)?)
}
