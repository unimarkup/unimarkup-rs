use super::ir::{IrTableName, RetrieveFromIr};
use crate::middleend::ir::{
    entry_already_exists, insert_ir_line_execute, update_ir_line_execute, WriteToIr,
};
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};

#[derive(Debug, PartialEq)]
pub struct VariableIrLine {
    pub name: String,
    pub um_type: String,
    pub value: String,
    pub fallback_value: String,
}

impl Default for VariableIrLine {
    fn default() -> Self {
        VariableIrLine {
            name: String::default(),
            um_type: String::default(),
            value: String::default(),
            fallback_value: String::default(),
        }
    }
}

impl IrTableName for VariableIrLine {
    fn table_name() -> String {
        "variables".to_string()
    }
}

impl VariableIrLine {
    pub fn new(
        name: impl Into<String>,
        um_type: impl Into<String>,
        value: impl Into<String>,
        fallback_value: impl Into<String>,
    ) -> Self {
        VariableIrLine {
            name: name.into(),
            um_type: um_type.into(),
            value: value.into(),
            fallback_value: fallback_value.into(),
        }
    }

    pub fn table_setup() -> String {
        r#"CREATE TABLE IF NOT EXISTS "variables" (
					"name"	TEXT NOT NULL,
					"um_type"	TEXT NOT NULL,
					"value"	TEXT,
					"fallback_value"	TEXT,
					PRIMARY KEY("name")
				);"#
        .to_string()
    }
}

impl WriteToIr for VariableIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = &VariableIrLine::table_name();
        let column_pk = format!("name: {}", self.name);
        let new_values = params![self.name, self.um_type, self.value, self.fallback_value,];

        let sql_exists_condition = "name = '?1'";
        let exists_params = params![self.name];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "name = ?1";
            let sql_set = "um_type = ?2, value = ?3, fallback_value = ?4";
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

impl RetrieveFromIr for VariableIrLine {
    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if row.as_ref().column_count() != 4 {
            return Err(InvalidParameterCount(row.as_ref().column_count(), 4));
        } else {
            Ok(VariableIrLine::new(
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, String>(2)?,
                row.get::<usize, String>(3)?,
            ))
        }
    }
}
