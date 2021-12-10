use super::super::middleend::ir_test_setup;
use unimarkup_rs::{frontend::parser_pest::parser_pest, um_error::UmError};

#[test]
fn test_file() -> Result<(), UmError> {
    let mut connection = ir_test_setup::setup_test_ir();

    parser_pest(&mut connection)?;
    Ok(())
}
