use crate::cli;
use crate::frontend;
use crate::middleend;
use crate::um_error::UmError;
// use crate::backend;

pub fn compile() -> Result<bool, UmError> { 
  let mut config = cli::get_config_from_cli();
  let mut connection = middleend::setup_ir_connection()?;

  // frontend::run(&connection, &config)?;
  // backend::run(&connection, &config)?;
  Ok(true)
}
