use unimarkup_rs::{frontend::parser_pest::{parser_pest, UnimarkupParser}, um_error::UmError, middleend::{self, ParseForIr},};
use super::super::middleend::ir_test_setup;

#[test]
fn test_file() -> Result<(), UmError> {

    let mut connection = ir_test_setup::setup_test_ir();

    parser_pest(&mut connection)?;
    Ok(())
}
