use logid::capturing::{LogIdTracing, MappedLogId};
use logid::log_id::LogId;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};
use rusqlite::{Connection, Statement, ToSql};
use std::convert::TryInto;

use crate::log_id::CORE_LOG_ID_MAP;
use crate::middleend::ir::{self, WriteToIr};
use crate::middleend::log_id::GeneralWarnLogId;

use super::ir::{IrTableName, RetrieveFromIr};
use super::{log_id::GeneralErrLogId, AsIrLines};

/// Structure for the content table representation of the IR
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContentIrLine {
    /// ID of the content
    pub id: String,
    /// Line number of the input Unimarkup file, where the start of this content is found.
    pub line_nr: usize,
    /// String representation of the [`UnimarkupType`] for this content.
    ///
    /// [`UnimarkupType`]: crate::um_elements::types::UnimarkupType
    pub um_type: String,
    /// Raw inline Unimarkup content for the [`UnimarkupType`].
    ///
    /// [`UnimarkupType`]: crate::um_elements::types::UnimarkupType
    pub text: String,
    /// Alternative content that is used, if `text` is empty.
    pub fallback_text: String,
    /// Attributes for this content in JSON format.
    pub attributes: String,
    /// Alternative attributes that are used, if `attributes` is empty.
    pub fallback_attributes: String,
}

impl Default for ContentIrLine {
    fn default() -> Self {
        ContentIrLine {
            id: String::from("0"),
            line_nr: 0,
            um_type: String::default(),
            text: String::default(),
            fallback_text: String::default(),
            attributes: String::default(),
            fallback_attributes: String::default(),
        }
    }
}

impl IrTableName for ContentIrLine {
    fn table_name() -> String {
        "content".to_string()
    }
}

impl ContentIrLine {
    /// Constructs a new [`ContentIrLine`].
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of a content
    /// * `line_nr` - Line number in the Unimarkup file, where this content is found
    /// * `um_type` - String representation of the [`UnimarkupType`]
    /// * `text` - Content of the [`ContentIrLine`]
    /// * `fallback_text` - Alternative content
    /// * `attributes` - Attributes of the [`ContentIrLine`]
    /// * `fallback_attributes` - Alternative attributes
    ///
    /// [`UnimarkupType`]: crate::um_elements::types::UnimarkupType
    pub fn new(
        id: impl Into<String>,
        line_nr: usize,
        um_type: impl Into<String>,
        text: impl Into<String>,
        fallback_text: impl Into<String>,
        attributes: impl Into<String>,
        fallback_attributes: impl Into<String>,
    ) -> Self {
        ContentIrLine {
            id: id.into(),
            line_nr,
            um_type: um_type.into(),
            text: text.into(),
            fallback_text: fallback_text.into(),
            attributes: attributes.into(),
            fallback_attributes: fallback_attributes.into(),
        }
    }

    /// Prepares a SQL query to setup the content table of the IR form.
    pub fn table_setup() -> String {
        r#"CREATE TABLE IF NOT EXISTS "content" (
					"id"	TEXT NOT NULL,
					"line_nr"	INTEGER NOT NULL,
					"um_type"	TEXT NOT NULL,
					"text"	TEXT,
					"fallback_text"	TEXT,
					"attributes"	TEXT,
					"fallback_attributes"	TEXT,
					PRIMARY KEY("id","line_nr")
				);"#
        .to_string()
    }
}

impl WriteToIr for ContentIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), MappedLogId> {
        let sql_table = &ContentIrLine::table_name();
        let column_pk = format!("id: {} at line: {}", self.id, self.line_nr);
        let new_values = params![
            self.id,
            self.line_nr,
            self.um_type,
            self.text,
            self.fallback_text,
            self.attributes,
            self.fallback_attributes,
        ];

        if ir::entry_already_exists(self, ir_transaction) {
            (GeneralWarnLogId::EntryOverwritten as LogId).set_event_with(
                &CORE_LOG_ID_MAP,
                &format!(
                    "Content with id: '{}' at line nr: '{}' is overwritten.",
                    self.id, self.line_nr
                ),
                file!(),
                line!(),
            );

            let sql_condition = "id = ?1 AND line_nr = ?2";
            let sql_set = "um_type = ?3, text = ?4, fallback_text = ?5, attributes = ?6, fallback_attributes = ?7";
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

impl RetrieveFromIr for ContentIrLine {
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>) {
        let sql_exists_condition = "id = ?1 AND line_nr = ?2";
        let exists_params = params![self.id, self.line_nr];
        (sql_exists_condition.to_string(), exists_params.to_vec())
    }

    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if row.as_ref().column_count() != 7 {
            return Err(InvalidParameterCount(row.as_ref().column_count(), 7));
        } else {
            let line_nr = row.get::<usize, i64>(1)?;
            Ok(ContentIrLine::new(
                row.get::<usize, String>(0)?,
                line_nr
                    .try_into()
                    .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(1, line_nr))?,
                row.get::<usize, String>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, String>(5)?,
                row.get::<usize, String>(6)?,
            ))
        }
    }
}

impl<T> WriteToIr for T
where
    T: AsIrLines<ContentIrLine>,
{
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), MappedLogId> {
        for line in self.as_ir_lines() {
            line.write_to_ir(ir_transaction)?;
        }

        Ok(())
    }
}

/// Prepares a SQL Statement to get [`ContentIrLine`]s ordered by line number
/// in ascending order
///
/// # Errors
///
/// Will return Err, if the SQL query cannot be converted to a C-compatible string, or if the underlying SQLite call fails.
pub fn prepare_content_rows(ir_connection: &Connection, order: bool) -> Result<Statement, Error> {
    let sql_order = if order { "ORDER BY line_nr ASC" } else { "" };
    let sql = format!(
        "SELECT * FROM {} {}",
        ContentIrLine::table_name(),
        sql_order
    );
    ir_connection.prepare(&sql)
}

/// Loads [`ContentIrLine`]s from the content table and returns them as a vector
///
/// # Arguments
///
/// * `connection` - [`rusqlite::Connection`] to interact with the IR
pub fn get_content_lines(connection: &mut Connection) -> Result<Vec<ContentIrLine>, MappedLogId> {
    let convert_err = |err| {
        (GeneralErrLogId::FailedRowQuery as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                "Failed to query content rows from IR.",
                file!(),
                line!(),
            )
            .add_info(&format!("Cause: {}", err))
    };

    let mut rows_statement = prepare_content_rows(connection, true).map_err(convert_err)?;

    let mut rows = rows_statement.query([]).map_err(convert_err)?;

    let mut lines: Vec<ContentIrLine> = Vec::new();

    while let Ok(Some(row)) = rows.next() {
        let content_ir = ContentIrLine::from_ir(row).map_err(convert_err)?;
        lines.push(content_ir);
    }

    Ok(lines)
}
