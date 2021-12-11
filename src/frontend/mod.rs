mod syntax_error;

use rusqlite::Connection;
pub use syntax_error::SyntaxError;

use crate::{config::Config, um_elements::types::UnimarkupBlock, um_error::UmError};

pub mod parser;

type UnimarkupBlocks = Vec<Box<dyn UnimarkupBlock>>;

pub fn run(_connection: &mut Connection, config: &mut Config) -> Result<(), UmError> {
    let _ = parser::parse_unimarkup(&config.um_file);

    Ok(())
}
