//! Structs, traits and helper functions for IR interaction between [`frontend`] and
//! [`backend`].
//!
//! [`backend`]: crate::backend
//! [`frontend`]: crate::frontend

mod block;
mod content;
mod macros;
mod metadata;
mod resources;
mod setup;
mod statements;
mod variables;

pub use block::*;
pub use content::*;
pub use macros::*;
pub use metadata::*;
pub use resources::*;
pub use setup::*;
pub use statements::*;
pub use variables::*;

pub mod log_id;

use logid::capturing::MappedLogId;
use rusqlite::{Error, Row, ToSql, Transaction};

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
