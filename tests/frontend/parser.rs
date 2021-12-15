use super::super::middleend::ir_test_setup;
use clap::StructOpt;
use unimarkup_rs::{
    config::Config,
    frontend::{self, parser::*},
    um_error::UmError,
};

#[test]
fn run() -> Result<(), UmError> {
    let mut connection = ir_test_setup::setup_test_ir();
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/all_syntax.um",
    ]);

    frontend::run(&mut connection, &mut cfg)?;

    Ok(())
}
