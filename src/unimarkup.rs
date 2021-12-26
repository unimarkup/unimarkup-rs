//! Entry module for unimarkup-rs.

use crate::backend;
use crate::config::Config;
use crate::frontend;
use crate::middleend;
use crate::um_error::UmError;

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns an [`UmError`], if error occurs during compilation.
pub fn compile(mut config: Config) -> Result<(), UmError> {
    let mut connection = middleend::setup_ir_connection()?;
    middleend::setup_ir(&connection)?;

    frontend::run(&mut connection, &mut config)?;
    backend::run(&mut connection, &config)?;
    Ok(())
}
