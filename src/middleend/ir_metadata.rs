use crate::middleend::{
    ir::{self, IrTableName, RetrieveFromIr, WriteToIr},
    IrError,
};
use crate::um_error::UmError;
use log::warn;
use rusqlite::ToSql;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};

/// Structure for the metadata table representation of the IR
#[derive(Debug, PartialEq, Default)]
pub struct MetadataIrLine {
    /// Generated hash code of a Unimarkup file.
    pub filehash: Vec<u8>,
    /// Filename of a Unimarkup file.
    pub filename: String,
    /// Path to a Unimarkup file.
    pub path: String,
    /// Preamble section of a Unimarkup file.
    pub preamble: String,
    /// Alternative preamble
    pub fallback_preamble: String,
    /// True, if the Unimarkup file is the entry point of a Unimarkup document, called `root`.
    pub root: bool,
}

impl IrTableName for MetadataIrLine {
    fn table_name() -> String {
        "metadata".to_string()
    }
}

impl MetadataIrLine {
    /// Constructs a new MetadataIrLine
    ///
    /// # Arguments
    ///
    /// * `filehash` - Generated hash code of a Unimarkup file
    /// * `filename` - Filename of a Unimarkup file
    /// * `path` - Path to a Unimarkup file
    /// * `preamble` - Preamble section of a Unimarkup file
    /// * `fallback_preamble` - Alternative preamble
    /// * `root` - A Unimarkup file is called `root`, if it is the entry point of a Unimarkup document
    pub fn new(
        filehash: Vec<u8>,
        filename: impl Into<String>,
        path: impl Into<String>,
        preamble: impl Into<String>,
        fallback_preamble: impl Into<String>,
        root: bool,
    ) -> Self {
        MetadataIrLine {
            filehash,
            filename: filename.into(),
            path: path.into(),
            preamble: preamble.into(),
            fallback_preamble: fallback_preamble.into(),
            root,
        }
    }

    /// Prepares a SQL query to setup the metadata table of the IR form.
    pub fn table_setup() -> String {
        r#"CREATE TABLE IF NOT EXISTS "metadata" (
					"filehash"	BLOB NOT NULL,
					"filename"	TEXT NOT NULL,
					"path"	TEXT NOT NULL,
					"preamble"	TEXT,
					"fallback_preamble"	TEXT,
					"root"	INTEGER,
					PRIMARY KEY("filehash")
				);"#
        .to_string()
    }
}

impl WriteToIr for MetadataIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmError> {
        let sql_table = &MetadataIrLine::table_name();
        let column_pk = format!(
            "filename: {} with hash: {}",
            self.filename,
            hex::encode_upper(&self.filehash),
        );
        let new_values = params![
            self.filehash.to_vec(),
            self.filename,
            self.path,
            self.preamble,
            self.fallback_preamble,
            self.root,
        ];

        if ir::entry_already_exists(self, ir_transaction) {
            warn!(
                "Metadata with filename: '{}' and path: '{}' is overwritten.",
                self.filename, self.path
            );
            let sql_condition = "filehash = ?1";
            let sql_set =
                "filename = ?2, path = ?3, preamble = ?4, fallback_preamble = ?5, root = ?6";
            ir::update_ir_line_execute(
                ir_transaction,
                sql_table,
                sql_set,
                sql_condition,
                new_values,
                &column_pk,
            )
        } else {
            ir::insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
        }
    }
}

impl RetrieveFromIr for MetadataIrLine {
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>) {
        let sql_exists_condition = "filehash = ?1";
        let exists_params = params![self.filehash];
        (sql_exists_condition.to_string(), exists_params.to_vec())
    }

    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if row.as_ref().column_count() != 6 {
            return Err(InvalidParameterCount(row.as_ref().column_count(), 6));
        } else {
            Ok(MetadataIrLine::new(
                row.get::<usize, Vec<u8>>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, String>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, bool>(5)?,
            ))
        }
    }
}
