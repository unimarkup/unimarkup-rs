use unimarkup_core::{backend::Render, elements::ParagraphBlock};
use unimarkup_inline::{parse_with_offset, Position};

#[test]
fn test__render_html__valid_escaped_inline() {
    let id = String::from("paragraph-id");
    let content = parse_with_offset("\\*23\\*3", Position::default()).unwrap();

    let mut block = ParagraphBlock {
        id: id.clone(),
        content,
        attributes: "{}".into(),
        line_nr: 0,
    };

    let mut expected_html = format!("<p id='{}'>*23*3</p>", id);

    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap(),
        "Html file does not match with expected output"
    );

    block.content = parse_with_offset("\\ *italic*\\", Position::default()).unwrap();
    expected_html = format!("<p id='{}'>&nbsp;<em>italic</em></p>", id);
    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap(),
        "Html file does not match with expected output"
    );

    block.content = parse_with_offset("**\\*only bold\\***", Position::default()).unwrap();
    expected_html = format!("<p id='{}'><strong>*only bold*</strong></p>", id);

    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap(),
        "Html file does not match with expected output"
    );
}
