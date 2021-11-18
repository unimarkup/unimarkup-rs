use crate::frontend::parser::CursorPos;
use crate::middleend::{ContentIrLine, IrBlock, IrError};
use crate::um_error::UmError;
use rusqlite::{Error, Row, ToSql, Transaction};

pub trait ParseForIr {
    fn parse_for_ir(
        content: &[&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmError>;

    fn generate_ir_lines(&self, line_nr: usize) -> Vec<ContentIrLine>;
}

pub trait IrTableName {
    fn table_name() -> String;
}

pub trait WriteToIr {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmError>;
}

pub trait RetrieveFromIr {
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>);
    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized + WriteToIr;
}

pub fn write_ir_lines(
    ir_lines: &[impl WriteToIr],
    ir_transaction: &Transaction,
) -> Result<(), UmError> {
    for ir_line in ir_lines {
        let res = ir_line.write_to_ir(ir_transaction);
        if res.is_err() {
            return Err(res.err().unwrap());
        }
    }
    Ok(())
}

pub fn entry_already_exists<T: IrTableName + RetrieveFromIr>(
    ir_line: &T,
    ir_transaction: &Transaction,
) -> bool {
    let (pk_condition, pk_values) = ir_line.get_pk_values();
    let sql = format!(
        "SELECT count(*) FROM {} WHERE {}",
        T::table_name(),
        pk_condition
    );
    let params: &[&dyn ToSql] = &pk_values;
    let res: Result<i64, Error> = ir_transaction.query_row(&sql, params, |row| row.get(0));
    if let Ok(cnt) = res {
        return cnt > 0;
    }
    false
}

fn get_nr_values(params: &[&dyn ToSql]) -> String {
    let mut s = String::new();
    for (i, _) in params.iter().enumerate() {
        s.push_str(&format!("?{},", i + 1));
    }
    s.pop(); // strip last ,
    s
}

pub fn insert_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), UmError> {
    let sql = format!(
        "INSERT INTO {} VALUES ({})",
        sql_table,
        get_nr_values(params)
    );

    let execute_res = ir_transaction.execute(&sql, params);
    if execute_res.is_err() {
        return Err(IrError::new(
            sql_table.to_string(),
            column.to_string(),
            format!(
                "Could not insert values on given database connection. Reason: {:?}",
                execute_res.err()
            ),
        )
        .into());
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
) -> Result<(), UmError> {
    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        sql_table, sql_set, sql_condition
    );

    let execute_res = ir_transaction.execute(&sql, params);
    if execute_res.is_err() {
        return Err(IrError::new(
            sql_table.to_string(),
            column.to_string(),
            format!(
                "Could not update values on given database connection. Reason: {:?}",
                execute_res.err()
            ),
        )
        .into());
    }
    Ok(())
}

pub fn get_single_ir_line<T: RetrieveFromIr + IrTableName + WriteToIr>(
    ir_transaction: &Transaction,
    pk_condition_params: (String, Vec<&dyn ToSql>),
) -> Result<T, UmError> {
    let sql = format!(
        "SELECT * FROM {} WHERE {}",
        T::table_name(),
        pk_condition_params.0
    );
    let params: &[&dyn ToSql] = &pk_condition_params.1;
    let res_query = ir_transaction.query_row(&sql, params, |row| T::from_ir(row));

    res_query.map_err(|err| {
        IrError::new(
            T::table_name(),
            pk_condition_params.0,
            format!(
                "Failed getting single IrLine from given database connection. Reason: {:?}",
                err
            ),
        )
        .into()
    })
}
