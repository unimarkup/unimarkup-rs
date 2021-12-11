use crate::backend;
use crate::config::Config;
use crate::frontend;
use crate::middleend;
use crate::um_error::UmError;

pub fn compile(mut config: Config) -> Result<(), UmError> {
    let mut connection = middleend::setup_ir_connection()?;

    frontend::run(&mut connection, &mut config)?;
    backend::run(&mut connection, &config)?;
    Ok(())
}
