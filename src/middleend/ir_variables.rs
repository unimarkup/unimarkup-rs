use crate::middleend::ir::{self, IrTableName, RetrieveFromIr, WriteToIr};
use crate::um_error::UmError;
use log::info;
use rusqlite::ToSql;
use rusqlite::{params, Error, Error::InvalidParameterCount, Row, Transaction};

#[derive(Debug, PartialEq, Default)]
pub struct VariableIrLine {
    pub name: String,
    pub um_type: String,
    pub value: String,
    pub fallback_value: String,
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
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmError> {
        let sql_table = &VariableIrLine::table_name();
        let column_pk = format!("name: {}", self.name);
        let new_values = params![self.name, self.um_type, self.value, self.fallback_value,];

        if ir::entry_already_exists(self, ir_transaction) {
            info!("Variable '{}' is overwritten.", self.name);
            let sql_condition = "name = ?1";
            let sql_set = "um_type = ?2, value = ?3, fallback_value = ?4";
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

impl RetrieveFromIr for VariableIrLine {
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>) {
        let sql_exists_condition = "name = ?1";
        let exists_params = params![self.name];
        (sql_exists_condition.to_string(), exists_params.to_vec())
    }

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
