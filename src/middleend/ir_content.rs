use super::ir::{IrTableName, RetrieveFromIr};
use crate::middleend::ir::{self, WriteToIr};
use crate::middleend::middleend_error::UmMiddleendError;
use log::warn;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};
use rusqlite::{Connection, Statement, ToSql};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct ContentIrLine {
    pub id: String,
    pub line_nr: usize,
    pub um_type: String,
    pub text: String,
    pub fallback_text: String,
    pub attributes: String,
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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
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
            Ok(ContentIrLine::new(
                row.get::<usize, String>(0)?,
                row.get::<usize, i64>(1)?.try_into().expect("Line number could not be converted to i64"),
                row.get::<usize, String>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, String>(5)?,
                row.get::<usize, String>(6)?,
            ))
        }
    }
}

pub fn prepare_content_rows(ir_connection: &Connection, order: bool) -> Result<Statement, Error> {
    let sql_order = if order { "ORDER BY line_nr ASC" } else { "" };
    let sql = format!(
        "SELECT * FROM {} {}",
        ContentIrLine::table_name(),
        sql_order
    );
    ir_connection.prepare(&sql)
}
