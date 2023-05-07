use unimarkup_commons::config::{output::OutputFormat, Config};
use unimarkup_core::frontend;

#[test]
fn test__frontend_run__all_syntax_valid() {
    let mut cfg = Config {
        input: "tests/test_files/all_syntax.um".into(),
        ..Default::default()
    };

    cfg.preamble.output.formats.insert(OutputFormat::Html);

    let input = std::fs::read_to_string(&cfg.input).unwrap();

    let result = frontend::run(&input, &mut cfg);
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
}
