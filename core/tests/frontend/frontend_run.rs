use super::super::middleend::ir_test_setup;
use clap::StructOpt;
use unimarkup_core::{config::Config, frontend};

#[test]
fn test__frontend_run__all_syntax_valid() {
    let mut connection = ir_test_setup::setup_test_ir();
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/all_syntax.um",
    ]);

    let input = std::fs::read_to_string(&cfg.um_file).unwrap();

    let result = frontend::run(&input, &mut connection, &mut cfg);
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
}
