use unimarkup_commons::config::{output::OutputFormat, Config};

#[test]
fn test__compile__empty_content() {
    let mut cfg = Config {
        input: "tests/test_files/all_syntax.um".into(),
        ..Default::default()
    };

    cfg.preamble.output.formats.insert(OutputFormat::Html);

    let rendered_result = unimarkup_core::unimarkup::compile("", cfg);

    assert!(rendered_result.blocks.is_empty());
}
