use super::ir::{IrTableName, RetrieveFromIr};
use super::IrError;
use crate::middleend::ir::{self, WriteToIr};
use crate::um_error::UmError;
use log::warn;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};
use rusqlite::{Connection, Statement, ToSql};
use std::convert::TryInto;

/// IR compatible representation of [`UnimarkupBlock`]
///
/// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
#[derive(Debug, PartialEq)]
pub struct ContentIrLine {
    /// Unique identifier of the given [`UnimarkupBlock`]. Is also used
    /// in rendering, e.g., as html id attribute.
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    pub id: String,
    /// Line number in input unimarkup file where this [`UnimarkupBlock`] is found.
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    pub line_nr: usize,
    /// String representation of [`UnimarkupType`]. May include suffix such as `"start"` for
    /// [`UnimarkupBlock`] which spans multiple [`ContentIrLine`]s.
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    /// [`UnimarkupType`]: crate::um_elements::types::UnimarkupType
    pub um_type: String,
    /// Content of the [`UnimarkupBlock`]. Content may be only partial in case where
    /// the given [`UnimarkupBlock`] spans multiple [`ContentIrLine`]s.
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    pub text: String,
    /// Alternative content of the [`UnimarkupBlock`].
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    pub fallback_text: String,
    /// Attributes of the [`UnimarkupBlock`], e.g., color, explicit id etc.
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    pub attributes: String,
    /// Alternative attributes.
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
    /// * `id` - The unique identifier
    /// * `line_nr` - Line number in unimarkup file this instance corresponds to
    /// * `um_type` - String representation of [`UnimarkupType`] (possibly with suffix)
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

    /// Constructs SQL query which prepares the IR for receiving of [`ContentIrLine`] data.
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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmError> {
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
            warn!(
                "Content with id: '{}' at line nr: '{}' is overwritten.",
                self.id, self.line_nr
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

/// Prepares SQL Statement for retrieving [`ContentIrLine`]s ordered by line number
/// in ascending order
///
/// # Errors
///
/// Will return Err if sql cannot be converted to a C-compatible string or if the underlying SQLite call fails.
pub fn prepare_content_rows(ir_connection: &Connection, order: bool) -> Result<Statement, Error> {
    let sql_order = if order { "ORDER BY line_nr ASC" } else { "" };
    let sql = format!(
        "SELECT * FROM {} {}",
        ContentIrLine::table_name(),
        sql_order
    );
    ir_connection.prepare(&sql)
}

/// Loads the [`ContentIrLine`]s from IR and returns them contained in a vector
///
/// # Arguments
/// * `connection` - [`rusqlite::Connection`] for interacting with IR
pub fn get_content_lines(connection: &mut Connection) -> Result<Vec<ContentIrLine>, UmError> {
    let convert_err = |err| -> UmError {
        IrError::new(
            ContentIrLine::table_name(),
            "unknown",
            format!("Failed to query row from IR. \nReason: {}", err),
        )
        .into()
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

/// Used to get [`ContentIrLine`]s representation of a [`UnimarkupBlock`]
///
/// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
pub trait AsIrLines {
    /// Constructs [`ContentIrLine`]s representation of [`UnimarkupBlock`]
    ///
    /// [`UnimarkupBlock`]: crate::um_elements::types::UnimarkupBlock
    fn as_ir_lines(&self) -> Vec<ContentIrLine>;
}
