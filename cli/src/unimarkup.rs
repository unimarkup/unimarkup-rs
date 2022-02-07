//! Entry module for unimarkup-rs.

use std::fs;

use unimarkup_core::config::Config;
use unimarkup_core::error::UmError;

/// Compiles a Unimarkup document.
///
/// # Arguments
///
/// * `config` - Unimarkup configuration constructed from command-line arguments passed to unimarkup-rs.
///
/// # Errors
///
/// Returns an [`UmError`], if error occurs during compilation.
pub fn compile(config: Config) -> Result<(), UmError> {
    let source = fs::read_to_string(&config.um_file).map_err(|err| UmError::General {
        msg: String::from("Could not read file."),
        error: Box::new(err),
    })?;

    let _document = unimarkup_core::unimarkup::compile(&source, config)?;

    // let mut connection = middleend::setup_ir_connection()?;
    // middleend::setup_ir(&connection)?;
    //
    // frontend::run(&mut connection, &mut config)?;
    // backend::run(&mut connection, &config)?;
    Ok(())
}
