use crate::{
    log_id::CORE_LOG_ID_MAP,
    middleend::{ContentIrLine, MacroIrLine, MetadataIrLine, ResourceIrLine, VariableIrLine},
};
use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use rusqlite::Connection;

use super::log_id::SetupErrLogId;

/// Creates the [`rusqlite::Connection`] to a in-memory SQLite database.
///
/// # Errors
///
/// Returns a [`MappedLogId`] if the connection could not be created.
pub fn setup_ir_connection() -> Result<Connection, MappedLogId> {
    Connection::open_in_memory().map_err(|err| {
        (SetupErrLogId::FailedDatabaseConnection as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                "Could not create a database connection.",
                file!(),
                line!(),
            )
            .add_cause(&format!("{}", err))
    })
}

/// Prepares all necessary tables for the IR form.
///
/// # Errors
///
/// Returns a [`MappedLogId`] if execution of a SQL statement fails.
pub fn setup_ir(ir_connection: &Connection) -> Result<(), MappedLogId> {
    let sql = format!(
        "{}{}{}{}{}",
        ContentIrLine::table_setup(),
        MacroIrLine::table_setup(),
        VariableIrLine::table_setup(),
        MetadataIrLine::table_setup(),
        ResourceIrLine::table_setup()
    );
    ir_connection.execute_batch(&sql).map_err(|err| {
        (SetupErrLogId::FailedTableCreation as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                "Could not setup database tables.",
                file!(),
                line!(),
            )
            .add_cause(&format!("{}", err))
    })
}
