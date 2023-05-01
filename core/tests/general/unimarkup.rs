use unimarkup_commons::config::{Config, OutputFormat};

#[test]
fn test__compile__empty_content() {
    let mut cfg = Config {
        input: "tests/test_files/all_syntax.um".into(),
        ..Default::default()
    };

    cfg.preamble.output.formats.insert(OutputFormat::Html);

    let rendered_result = unimarkup_core::unimarkup::compile("", cfg);

    assert!(rendered_result.unwrap().blocks.is_empty());
}
