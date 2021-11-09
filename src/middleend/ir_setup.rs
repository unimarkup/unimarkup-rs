use crate::middleend::{
    ir_content::ContentIrLine, ir_macros::MacroIrLine, ir_metadata::MetadataIrLine,
    ir_resources::ResourceIrLine, ir_variables::VariableIrLine, middleend_error::UmMiddleendError,
};
use rusqlite::Connection;

pub fn setup_ir_connection() -> Result<Connection, UmMiddleendError> {
    let connection = Connection::open_in_memory();
    if connection.is_err() {
        return Err(UmMiddleendError {
            tablename: "-".to_string(),
            column: "-".to_string(),
            message: "Could not create a database connection".to_string(),
        });
    }
    Ok(connection.unwrap())
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
    let setup_res = ir_connection.execute_batch(&sql);

    if setup_res.is_err() {
        return Err(UmMiddleendError {
            tablename: "-".to_string(),
            column: "-".to_string(),
            message: "Could not setup tables on given database connection".to_string(),
        });
    }
    Ok(())
}
