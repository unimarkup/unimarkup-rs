use super::ir::IrTableName;
use crate::middleend::ir::{entry_already_exists, insert_ir_line_execute, WriteToIr};
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{params, Transaction};

#[derive(Debug)]
pub struct ResourceIrLine {
    pub filename: String,
    pub path: String,
}

impl Default for ResourceIrLine {
    fn default() -> Self {
        ResourceIrLine {
            filename: String::default(),
            path: String::default(),
        }
    }
}

impl IrTableName for ResourceIrLine {
    fn table_name() -> String {
        "resources".to_string()
    }
}

impl ResourceIrLine {
    pub fn new(filename: impl Into<String>, path: impl Into<String>) -> Self {
        ResourceIrLine {
            filename: filename.into(),
            path: path.into(),
        }
    }

    pub fn table_setup() -> String {
        r#"CREATE TABLE IF NOT EXISTS "resources" (
					"filename"	TEXT NOT NULL,
					"path"	TEXT NOT NULL,
					PRIMARY KEY("path","filename")
				);"#
        .to_string()
    }
}

impl WriteToIr for ResourceIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = &ResourceIrLine::table_name();
        let column_pk = format!("filename: {} with path: {}", self.filename, self.path);
        let new_values = params![self.filename, self.path];

        let sql_exists_condition = "filename = ?1 AND path = ?2";

        if entry_already_exists(ir_transaction, sql_table, sql_exists_condition, new_values) {
            // All resources columns are used for private key, no update needed
            return Ok(());
        }
        insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
    }
}
