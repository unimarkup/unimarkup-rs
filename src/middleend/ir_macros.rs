use crate::middleend::ir::{
    entry_already_exists, insert_ir_line_execute, update_ir_line_execute, WriteToIr,
};
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{params, Transaction};

#[derive(Debug)]
pub struct MacroIrLine {
    pub name: String,
    pub um_type: String,
    pub parameters: String,
    pub body: String,
    pub fallback_body: String,
}

impl Default for MacroIrLine {
    fn default() -> Self {
        MacroIrLine {
            name: String::default(),
            um_type: String::default(),
            parameters: String::default(),
            body: String::default(),
            fallback_body: String::default(),
        }
    }
}

impl MacroIrLine {
    pub fn new(
        name: impl Into<String>,
        um_type: impl Into<String>,
        parameters: impl Into<String>,
        body: impl Into<String>,
        fallback_body: impl Into<String>,
    ) -> Self {
        MacroIrLine {
            name: name.into(),
            um_type: um_type.into(),
            parameters: parameters.into(),
            body: body.into(),
            fallback_body: fallback_body.into(),
        }
    }

    pub fn table_setup() -> String {
        r#"CREATE TABLE IF NOT EXISTS "macros" (
					"name"	TEXT NOT NULL,
					"parameters"	BLOB NOT NULL,
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
        let sql_table = "macros";
        let column_pk = format!("name: {} with parameters: {}", self.name, self.parameters);
        let new_values = params![
            self.name,
            self.parameters,
            self.um_type,
            self.body,
            self.fallback_body,
        ];

        let sql_exists_condition = "name = ?1 AND parameters = ?2";
        let exists_params = params![self.name, self.parameters];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "name = ?1 AND parameters = ?2";
            let sql_set = "um_type = ?3, body = ?4, fallback_body = ?5";
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
