use crate::{
    middleend::{
        ContentIrLine, IrError, MacroIrLine, MetadataIrLine, ResourceIrLine, VariableIrLine,
    },
    um_error::UmError,
};
use rusqlite::Connection;

/// Creates the [`rusqlite::Connection`] in memory.
///
/// # Errors
///
/// Returns a [`UmError::Ir`] if connection could not be created.
pub fn setup_ir_connection() -> Result<Connection, UmError> {
    Connection::open_in_memory().map_err(|err| {
        IrError::new(
            "-".to_string(),
            "-".to_string(),
            format!("Could not create a database connection. Reason: {:?}", err),
        )
        .into()
    })
}

/// Prepares all the necessary tables in IR for receiving data.
///
/// # Errors
///
/// Returns a [`UmError::Ir`] if execution of SQL statements fails.
pub fn setup_ir(ir_connection: &Connection) -> Result<(), UmError> {
    let sql = format!(
        "{}{}{}{}{}",
        ContentIrLine::table_setup(),
        MacroIrLine::table_setup(),
        VariableIrLine::table_setup(),
        MetadataIrLine::table_setup(),
        ResourceIrLine::table_setup()
    );
    ir_connection.execute_batch(&sql).map_err(|err| {
        IrError::new(
            "-".to_string(),
            "-".to_string(),
            format!(
                "Could not setup tables on given database connection. Reason: {:?}",
                err
            ),
        )
        .into()
    })
}
