use crate::{
    middleend::{
        ContentIrLine, MacroIrLine, MetadataIrLine, ResourceIrLine, VariableIrLine,
    }, log_id::{LogId, SetLog},
};
use rusqlite::Connection;

use super::{error::MiddleendError, log_id::SetupErrLogId};

/// Creates the [`rusqlite::Connection`] to a in-memory SQLite database.
///
/// # Errors
///
/// Returns a [`UmError::Ir`], if the connection could not be created.
pub fn setup_ir_connection() -> Result<Connection, MiddleendError> {
    Connection::open_in_memory().map_err(|err| {
        MiddleendError::Setup(
            (SetupErrLogId::FailedDatabaseConnection as LogId).set_log(
                "Could not create a database connection.",
                file!(),
                line!()
            ).add_to_log(&format!("Cause: {}", err))
        )
    })
}

/// Prepares all necessary tables for the IR form.
///
/// # Errors
///
/// Returns a [`MiddleendError`], if execution of a SQL statement fails.
pub fn setup_ir(ir_connection: &Connection) -> Result<(), MiddleendError> {
    let sql = format!(
        "{}{}{}{}{}",
        ContentIrLine::table_setup(),
        MacroIrLine::table_setup(),
        VariableIrLine::table_setup(),
        MetadataIrLine::table_setup(),
        ResourceIrLine::table_setup()
    );
    ir_connection.execute_batch(&sql).map_err(|err| {
        MiddleendError::Setup(
            (SetupErrLogId::FailedTableCreation as LogId).set_log(
                "Could not setup database tables.",
                file!(),
                line!()
            ).add_to_log(&format!("Cause: {}", err))
        )
    })
}
