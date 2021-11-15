use crate::middleend::{
    ir_content::ContentIrLine, ir_macros::MacroIrLine, ir_metadata::MetadataIrLine,
    ir_resources::ResourceIrLine, ir_variables::VariableIrLine, middleend_error::UmMiddleendError,
};
use rusqlite::Connection;

pub fn setup_ir_connection() -> Result<Connection, UmMiddleendError> {
    Connection::open_in_memory().map_err(|_| UmMiddleendError {
        tablename: "-".to_string(),
        column: "-".to_string(),
        message: "Could not create a database connection".to_string(),
    })
}

pub fn setup_ir(ir_connection: &Connection) -> Result<(), UmMiddleendError> {
    let sql = format!(
        "{}{}{}{}{}",
        ContentIrLine::table_setup(),
        MacroIrLine::table_setup(),
        VariableIrLine::table_setup(),
        MetadataIrLine::table_setup(),
        ResourceIrLine::table_setup()
    );
    ir_connection
        .execute_batch(&sql)
        .map_err(|_| UmMiddleendError {
            tablename: "-".to_string(),
            column: "-".to_string(),
            message: "Could not setup tables on given database connection".to_string(),
        })
}
