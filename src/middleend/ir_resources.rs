use crate::middleend::ir::{self, IrTableName, RetrieveFromIr, WriteToIr};
use crate::um_error::UmError;
use log::debug;
use rusqlite::ToSql;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};

/// IR compatible representation of unimarkup Resource elements.
#[derive(Debug, PartialEq, Default)]
pub struct ResourceIrLine {
    /// File name of the given resource, i.e. name of image file.
    pub filename: String,
    /// Path to the given resource.
    pub path: String,
}

impl IrTableName for ResourceIrLine {
    fn table_name() -> String {
        "resources".to_string()
    }
}

impl ResourceIrLine {
    /// Constructs a new [`ResourceIrLine`].
    ///
    /// # Arguments
    /// * `filename` - File name of the given resource, i.e. name of image file.
    /// * `path` - Path to the given resource.
    pub fn new(filename: impl Into<String>, path: impl Into<String>) -> Self {
        ResourceIrLine {
            filename: filename.into(),
            path: path.into(),
        }
    }

    /// Constructs SQL query which prepares the IR for receiving of [`ResourceIrLine`] data.
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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmError> {
        let sql_table = &ResourceIrLine::table_name();
        let column_pk = format!("filename: {} with path: {}", self.filename, self.path);
        let new_values = params![self.filename, self.path];

        if ir::entry_already_exists(self, ir_transaction) {
            // All resources columns are used for private key, no update needed
            debug!(
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
