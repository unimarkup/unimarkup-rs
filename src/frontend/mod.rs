mod syntax_error;

use rusqlite::Connection;
pub use syntax_error::SyntaxError;

use crate::{
    config::Config, middleend::WriteToIr, um_elements::types::UnimarkupBlock, um_error::UmError,
};

pub mod parser;

pub(crate) type UnimarkupBlocks = Vec<Box<dyn UnimarkupBlock>>;

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
