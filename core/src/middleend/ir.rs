use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use rusqlite::{Error, Row, ToSql, Transaction};

use crate::log_id::CORE_LOG_ID_MAP;

use super::log_id::GeneralErrLogId;

/// Used to get the table name of the given IR line structure
pub trait IrTableName {
    /// Returns the table name associated with the given IR line structure.
    /// i.e. "content" for [`ContentIrLine`]
    ///
    /// [`ContentIrLIne`]: (crate::ir::ContentIrLine)
    fn table_name() -> String;
}

/// Used to write the given IR line structure into IR
pub trait WriteToIr {
    /// Writes the structure into IR.
    ///
    /// # Errors
    ///
    /// Returns a [`MappedLogId`] if writing to IR fails.
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), MappedLogId>;
}

/// Trait to represent a Unimarkup struct as a type that is writable to IR.
pub trait AsIrLines<T>
where
    T: WriteToIr,
{
    /// Returns a Unimarkup struct as a type that is writable to IR.
    fn as_ir_lines(&self) -> Vec<T>;
}

impl<T> WriteToIr for Vec<T>
where
    T: WriteToIr,
{
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), MappedLogId> {
        for element in self {
            element.write_to_ir(ir_transaction)?;
        }

        Ok(())
    }
}

/// Used to retrieve a IR line structure from IR.
pub trait RetrieveFromIr {
    /// Gets the primary key (pk) values of the IR line structure,
    /// together with the SQL query, to fetch the
    /// identified structure from IR.
    fn get_pk_values(&self) -> (String, Vec<&dyn ToSql>);

    /// Fetches and generates `Self` (i.e. IR line) from IR.
    fn from_ir(row: &Row) -> Result<Self, Error>
    where
        Self: Sized + WriteToIr;
}

/// Writes IR lines into IR.
///
/// **Note:** The transaction must be commited manually, before the SQL database is updated.
///
/// # Arguments
///
/// * `ir_lines` - IR lines to write into the IR
/// * `ir_transaction` - the [`Transaction`] used to communicate with IR
///
/// # Errors
///
/// Returns a [`MappedLogId`] if writing an IR line to IR fails.
///
/// [`Transaction`]: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
pub fn write_ir_lines(
    ir_lines: &[impl WriteToIr],
    ir_transaction: &Transaction,
) -> Result<(), MappedLogId> {
    for ir_line in ir_lines {
        let res = ir_line.write_to_ir(ir_transaction);
        if res.is_err() {
            return Err(res.err().unwrap());
        }
    }
    Ok(())
}

/// Checks if an entry already exists in IR
///
/// **Note:** The transaction must be commited manually, before the SQL database is updated.
///
/// # Arguments
///
/// * `ir_line` - the IR line that is checked
/// * `ir_transaction` - the rusqlite [`Transaction`] used to communicate with IR
///
/// [`Transaction`]: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
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

/// Inserts the given IR line into the IR database.
///
/// **Note:** The transaction must be commited manually, before the SQL database is updated.
///
/// # Arguments
///
/// * `ir_transaction` - rusqlite [`Transaction`] used to communicate with IR
/// * `sql_table` - the table in IR to write to
/// * `params` - parameters for the SQL query, that will be inserted
/// * `column` - column associated with the value(s), that will be inserted.
///
/// # Errors
///
/// Returns a [`MappedLogId`] if insertion into IR fails.
///
/// [`Transaction`]: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
pub fn insert_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), MappedLogId> {
    let sql = format!(
        "INSERT INTO {} VALUES ({})",
        sql_table,
        get_nr_values(params)
    );

    let execute_res = ir_transaction.execute(&sql, params);
    if execute_res.is_err() {
        return Err((GeneralErrLogId::FailedValueInsert as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                &format!(
                    "Could not insert values '{}' into table '{}'.",
                    column, sql_table
                ),
                file!(),
                line!(),
            )
            .add_info(&format!("Cause: {:?}", execute_res.err())));
    }
    Ok(())
}

/// Updates the value in IR, i.e. when overriding some definition
///
/// **Note:** The transaction must be commited manually, before the SQL database is updated.
///
/// # Arguments
///
/// * `ir_transaction` - rusqlite [`Transaction`] used to communicate with IR
/// * `sql_table` - table in IR to write to
/// * `sql_set` - columns and values for the `SQL SET` command
/// * `sql_condition` - condition which identifies the row, that will be updated
/// * `params` - parameters for the SQL query, that will be updated
/// * `column` - column associated with the value(s), that will be inserted
///
/// # Errors
///
/// Returns a [`MappedLogId`] if updating values in IR fails.
///
/// [`Transaction`]: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
pub fn update_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    sql_set: &str,
    sql_condition: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), MappedLogId> {
    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        sql_table, sql_set, sql_condition
    );

    let execute_res = ir_transaction.execute(&sql, params);
    if execute_res.is_err() {
        return Err((GeneralErrLogId::FailedValueUpdate as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                &format!(
                    "Could not update values '{}' for table '{}'.",
                    column, sql_table
                ),
                file!(),
                line!(),
            )
            .add_info(&format!("Cause: {:?}", execute_res.err())));
    }
    Ok(())
}

/// Returns a single IR line from IR database.
///
/// **Note:** The transaction must be commited manually, before the SQL database is updated.
///
/// # Arguments
///
/// * `ir_transaction` - rusqlite [`Transaction`] used to communicate with IR
/// * `pk_condition_params` - SQL params to identify the IR line (Row in IR) to fetch
///
/// # Errors
///
/// Returns a [`MappedLogId`] if communication with IR fails.
///
/// [`Transaction`]: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
pub fn get_single_ir_line<T: RetrieveFromIr + IrTableName + WriteToIr>(
    ir_transaction: &Transaction,
    pk_condition_params: (String, Vec<&dyn ToSql>),
) -> Result<T, MappedLogId> {
    let sql = format!(
        "SELECT * FROM {} WHERE {}",
        T::table_name(),
        pk_condition_params.0
    );
    let params: &[&dyn ToSql] = &pk_condition_params.1;
    let res_query = ir_transaction.query_row(&sql, params, |row| T::from_ir(row));

    res_query.map_err(|err| {
        (GeneralErrLogId::FailedRowQuery as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                &format!(
                    "Failed getting single IrLine from table `{}`.",
                    T::table_name()
                ),
                file!(),
                line!(),
            )
            .add_info(&format!("PK condition: {}", pk_condition_params.0))
            .add_info(&format!("Cause: {:?}", err))
    })
}
