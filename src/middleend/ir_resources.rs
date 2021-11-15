use super::ir::{IrTableName, RetrieveFromIr};
use crate::middleend::ir::{self, WriteToIr};
use crate::middleend::middleend_error::UmMiddleendError;
use log::info;
use rusqlite::ToSql;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};

#[derive(Debug, PartialEq)]
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

        if ir::entry_already_exists(self, ir_transaction) {
            // All resources columns are used for private key, no update needed
            info!(
                "Resource with filename: '{}' and path: '{}' already in IR.",
                self.filename, self.path
            );
            return Ok(());
        }
        ir::insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
    }
}

impl RetrieveFromIr for ResourceIrLine {
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>) {
        let sql_exists_condition = "filename = ?1 AND path = ?2";
        let exists_params = params![self.filename, self.path];
        (sql_exists_condition.to_string(), exists_params.to_vec())
    }

    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if row.as_ref().column_count() != 2 {
            return Err(InvalidParameterCount(row.as_ref().column_count(), 2));
        } else {
            Ok(ResourceIrLine::new(
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
            ))
        }
    }
}
