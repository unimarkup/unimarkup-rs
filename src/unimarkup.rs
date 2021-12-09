use crate::config::Config;
// use crate::frontend;
use crate::middleend;
use crate::um_error::UmError;
// use crate::backend;

pub fn compile(_config: Config) -> Result<(), UmError> {
    let mut _connection = middleend::setup_ir_connection()?;

    // frontend::run(&connection, &mut config)?;
    // backend::run(&connection, &config)?;
    Ok(())
}
