//! Frontend functionality of [`unimarkup-rs`](crate).
//!
//! i.e. parsing of unimarkup-rs files, generating corresponding
//! ['UnimarkupBlocks'] and sending them to the IR.

mod syntax_error;

use rusqlite::Connection;
pub use syntax_error::SyntaxError;

use crate::{
    config::Config, middleend::WriteToIr, um_elements::types::UnimarkupBlock, um_error::UmError,
};

pub mod parser;

/// Type alias for a vector of elements that implement the [`UnimarkupBlock`] trait.
pub type UnimarkupBlocks = Vec<Box<dyn UnimarkupBlock>>;

/// `frontend::run` is the entry function of the [`frontend`] module.
/// It parses a Unimarkup file and sends the data to the IR.
///
/// # Errors
///
/// This function will return an error if the given Unimarkup file contains invalid syntax,
/// or if communication with IR fails.
///
/// [`frontend`]: crate::frontend
pub fn run(connection: &mut Connection, config: &mut Config) -> Result<(), UmError> {
    let blocks = parser::parse_unimarkup(&config.um_file)?;

    let transaction = connection.transaction();

    if let Ok(transaction) = transaction {
        for block in blocks {
            for ir_line in block.as_ir_lines() {
                //TODO: add filename to id
                ir_line.write_to_ir(&transaction)?;
            }
        }

        let _ = transaction.commit();
    }

    Ok(())
}
