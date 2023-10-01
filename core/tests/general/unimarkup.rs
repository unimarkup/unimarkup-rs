use unimarkup_commons::config::{output::OutputFormatKind, Config};

pub fn compile_empty_content() {
    let mut cfg = Config {
        input: "tests/test_files/all_syntax.um".into(),
        ..Default::default()
    };

    cfg.output.formats.insert(OutputFormatKind::Html);

    let rendered_result = unimarkup_core::Unimarkup::parse("", cfg);

    assert!(rendered_result.get_document().blocks.is_empty());
}
