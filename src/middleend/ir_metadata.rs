use crate::middleend::ir::{
    entry_already_exists, insert_ir_line_execute, update_ir_line_execute, WriteToIr,
};
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{params, Transaction};
use serde_bytes::ByteBuf;

#[derive(Debug)]
pub struct MetadataIrLine {
    pub filename: String,
    pub filehash: ByteBuf,
    pub path: String,
    pub preamble: String,
    pub fallback_preamble: String,
    pub root: bool,
}

impl Default for MetadataIrLine {
    fn default() -> Self {
        MetadataIrLine {
            filename: String::default(),
            filehash: ByteBuf::new(),
            path: String::default(),
            preamble: String::default(),
            fallback_preamble: String::default(),
            root: false,
        }
    }
}

impl MetadataIrLine {
    pub fn new(
        filename: impl Into<String>,
        filehash: ByteBuf,
        path: impl Into<String>,
        preamble: impl Into<String>,
        fallback_preamble: impl Into<String>,
        root: bool,
    ) -> Self {
        MetadataIrLine {
            filename: filename.into(),
            filehash,
            path: path.into(),
            preamble: preamble.into(),
            fallback_preamble: fallback_preamble.into(),
            root,
        }
    }

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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = "metadata";
        let column_pk = format!(
            "filename: {} with hash: {}",
            self.filename,
            String::from_utf8(self.filehash.to_vec()).unwrap()
        );
        let new_values = params![
            self.filehash.to_vec(),
            self.filename,
            self.path,
            self.preamble,
            self.fallback_preamble,
            self.root,
        ];

        let sql_exists_condition = "filehash = ?1";
        let exists_params = params![self.filehash.to_vec()];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "filehash = ?1";
            let sql_set =
                "filename = ?2, path = ?3, preamble = ?4, fallback_preamble = ?5, root = ?6";
            update_ir_line_execute(
                ir_transaction,
                sql_table,
                sql_set,
                sql_condition,
                new_values,
                &column_pk,
            )
        } else {
            insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
        }
    }
}
