//! Entry module for the unimarkup-rs compiler.

use crate::backend;
use crate::config::Config;
use crate::frontend;
use crate::middleend;
use crate::um_error::UmError;

/// Compiles unimarkup document.
///
/// # Arguments
/// * `config` - Configuration constructed from command-line arguments passed to the compiler.
///
/// # Errors
///
/// Returns an [`UmError`] if error occurs in either [`frontend`], [`middleend`] or
/// [`backend`] of the compiler.
///
/// [`frontend`]: crate::frontend
/// [`middleend`]: crate::middleend
/// [`backend`]: crate::backend
pub fn compile(mut config: Config) -> Result<(), UmError> {
    let mut connection = middleend::setup_ir_connection()?;
    middleend::setup_ir(&connection)?;

    frontend::run(&mut connection, &mut config)?;
    backend::run(&mut connection, &config)?;
    Ok(())
}
