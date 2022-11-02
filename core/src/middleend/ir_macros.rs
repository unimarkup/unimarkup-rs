use logid::capturing::{LogIdTracing, MappedLogId};
use logid::log_id::LogId;
use rusqlite::ToSql;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};

use crate::log_id::CORE_LOG_ID_MAP;
use crate::middleend::ir::{self, WriteToIr};

use super::ir::{IrTableName, RetrieveFromIr};
use super::log_id::GeneralInfLogId;

/// Structure for the macro table representation of the IR
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct MacroIrLine {
    /// Name of the macro.
    pub name: String,
    /// Parameters of the macro.
    pub parameters: String,
    /// The return type of the macro ([`UnimarkupType`] as [`String`]).
    ///
    /// [`UnimarkupType`]: crate::elements::types::UnimarkupType
    pub um_type: String,
    /// Macro definition.
    pub body: String,
    /// Alternative macro definition.
    pub fallback_body: String,
}

impl IrTableName for MacroIrLine {
    fn table_name() -> String {
        "macros".to_string()
    }
}

impl MacroIrLine {
    /// Constructs a new MacroIrLine
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the macro
    /// * `parameters` - Parameters of the macro
    /// * `um_type` - The return type of the macro ([`UnimarkupType`] as [`String`])
    /// * `body` - Macro definition
    /// * `fallback_body` - Alternative macro definition
    ///
    /// [`UnimarkupType`]: crate::elements::types::UnimarkupType
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

    /// Prepares a SQL query to setup the macro table of the IR form.
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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), MappedLogId> {
        let sql_table = &MacroIrLine::table_name();
        let column_pk = format!("name: {} with parameters: {}", self.name, self.parameters);
        let new_values = params![
            self.name,
            self.parameters,
            self.um_type,
            self.body,
            self.fallback_body,
        ];

        if ir::entry_already_exists(self, ir_transaction) {
            (GeneralInfLogId::EntryOverwritten as LogId).set_event_with(
                &CORE_LOG_ID_MAP,
                &format!(
                    "Macro with name: '{}' and parameters: '{}' is overwritten.",
                    self.name, self.parameters
                ),
                file!(),
                line!(),
            );

            let sql_condition = "name = ?1 AND parameters = ?2";
            let sql_set = "um_type = ?3, body = ?4, fallback_body = ?5";
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

impl RetrieveFromIr for MacroIrLine {
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>) {
        let sql_exists_condition = "name = ?1 AND parameters = ?2";
        let exists_params = params![self.name, self.parameters];
        (sql_exists_condition.to_string(), exists_params.to_vec())
    }

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
                row.get::<usize, String>(4)?,
            ))
        }
    }
}
