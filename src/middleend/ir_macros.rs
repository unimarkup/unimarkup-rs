use super::ir::{IrTableName, RetrieveFromIr};
use crate::middleend::ir::{
    entry_already_exists, insert_ir_line_execute, update_ir_line_execute, WriteToIr,
};
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{Error, Row, Transaction, params, Error::InvalidParameterCount};

#[derive(Debug, PartialEq)]
pub struct MacroIrLine {
    pub name: String,
    pub parameters: String,
    pub um_type: String,
    pub body: String,
    pub fallback_body: String,
}

impl Default for MacroIrLine {
    fn default() -> Self {
        MacroIrLine {
            name: String::default(),
            parameters: String::default(),
            um_type: String::default(),
            body: String::default(),
            fallback_body: String::default(),
        }
    }
}

impl IrTableName for MacroIrLine {
    fn table_name() -> String {
        "macros".to_string()
    }
}

impl MacroIrLine {
    pub fn new(
        name: impl Into<String>,
        parameters: impl Into<String>,
        um_type: impl Into<String>,
        body: impl Into<String>,
        fallback_body: impl Into<String>,
    ) -> Self {
        MacroIrLine {
            name: name.into(),
            parameters: parameters.into(),
            um_type: um_type.into(),
            body: body.into(),
            fallback_body: fallback_body.into(),
        }
    }

    pub fn table_setup() -> String {
        r#"CREATE TABLE IF NOT EXISTS "macros" (
            "name"	TEXT NOT NULL,
            "parameters"	TEXT NOT NULL,
            "um_type"	TEXT NOT NULL,
            "body"	TEXT,
            "fallback_body"	TEXT,
            PRIMARY KEY("name","parameters")
        );"#
        .to_string()
    }
}

impl WriteToIr for MacroIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = &MacroIrLine::table_name();
        let column_pk = format!("name: {} with parameters: {}", self.name, self.parameters);
        let new_values = params![
            self.name,
            self.parameters,
            self.um_type,
            self.body,
            self.fallback_body,
        ];

        let sql_exists_condition = "name = '?1' AND parameters = '?2'";
        let exists_params = params![self.name, self.parameters];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "name = '?1' AND parameters = '?2'";
            let sql_set = "um_type = '?3', body = '?4', fallback_body = '?5'";
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

impl RetrieveFromIr for MacroIrLine {
    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if row.as_ref().column_count() != 5 {
            return Err(InvalidParameterCount(row.as_ref().column_count(), 5));
        } else {
            Ok(MacroIrLine::new(
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, String>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?                
            ))
        }
    }
}
