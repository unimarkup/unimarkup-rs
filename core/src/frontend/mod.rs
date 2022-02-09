//! Frontend functionality of [`unimarkup-rs`](crate).
//!
//! i.e. parsing of unimarkup-rs files, generating corresponding
//! ['UnimarkupBlocks'] and sending them to the IR.

use rusqlite::Connection;

use crate::{config::Config, middleend::{WriteToIr, log_id::GeneralInfLogId}, log_id::{SetLog, LogId}};

use self::error::FrontendError;

pub mod parser;
pub mod preamble;
pub mod error;
pub mod log_id;

/// `frontend::run` is the entry function of the [`frontend`] module.
/// It parses a Unimarkup file and sends the data to the IR.
///
/// # Errors
///
/// This function will return an error if the given Unimarkup file contains invalid syntax,
/// or if communication with IR fails.
///
/// [`frontend`]: crate::frontend
pub fn run(
    um_content: &str,
    connection: &mut Connection,
    config: &mut Config,
) -> Result<(), FrontendError> {
    let unimarkup = parser::parse_unimarkup(um_content, config)?;

    let transaction = connection.transaction();

    if let Ok(transaction) = transaction {
        unimarkup.blocks.write_to_ir(&transaction)?;

        for metadata in unimarkup.metadata {
            metadata.write_to_ir(&transaction)?;
        }

        let _ = transaction.commit();
    }
    (GeneralInfLogId::EntryOverwritten as LogId).set_log("Test Log---------------------", file!(), line!());
    Ok(())
}
