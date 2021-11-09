use crate::frontend::{parser::CursorPos, syntax_error::UmSyntaxError};
use crate::middleend::ir_block::IrBlock;
use crate::middleend::ir_content::ContentIrLine;
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{ToSql, Transaction};

pub trait ParseForIr {
    fn parse_for_ir(
        content: &[&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmSyntaxError>;

    fn generate_ir_lines(&self, line_nr: usize) -> Vec<ContentIrLine>;
}

pub trait IrTableName {
    fn table_name() -> String;
}

pub trait WriteToIr {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError>;
}

pub fn write_ir_lines(
    ir_lines: &[impl WriteToIr],
    ir_transaction: &Transaction,
) -> Result<(), UmMiddleendError> {
    for ir_line in ir_lines {
        let res = ir_line.write_to_ir(ir_transaction);
        if res.is_err() {
            return Err(res.err().unwrap());
        }
    }
    Ok(())
}

pub fn entry_already_exists(
    ir_transaction: &Transaction,
    sql_table: &str,
    sql_condition: &str,
    params: &[&dyn ToSql],
) -> bool {
    let sql = format!(
        "SELECT count(*) FROM {} WHERE {} VALUES (?)",
        sql_table, sql_condition
    );
    let res: Result<i64, rusqlite::Error> =
        ir_transaction.query_row(&sql, params, |row| row.get(0));
    if res.is_ok() {
        return true;
    }
    false
}

pub fn insert_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), UmMiddleendError> {
    let sql = format!("INSERT INTO {} VALUES (?)", sql_table);

    if ir_transaction.execute(&sql, params).is_err() {
        return Err(UmMiddleendError {
            tablename: sql_table.to_string(),
            column: column.to_string(),
            message: "Could not insert values on given database connection".to_string(),
        });
    }
    Ok(())
}

pub fn update_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    sql_set: &str,
    sql_condition: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), UmMiddleendError> {
    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        sql_table, sql_set, sql_condition
    );

    if ir_transaction.execute(&sql, params).is_err() {
        return Err(UmMiddleendError {
            tablename: sql_table.to_string(),
            column: column.to_string(),
            message: "Could not update values on given database connection".to_string(),
        });
    }
    Ok(())
}

