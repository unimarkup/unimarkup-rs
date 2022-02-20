use unimarkup_core::{backend::Render, elements::ParagraphBlock};

#[test]
fn test__render_html__valid_escaped_inline() {
    let id = String::from("paragraph-id");
    let content = String::from("\\*23\\*3");

    let mut block = ParagraphBlock {
        id: id.clone(),
        content,
        attributes: "{}".into(),
        line_nr: 0,
    };

    let mut expected_html = format!("<p id='{}'>\\*23\\*3</p>", id);

    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap(),
        "Html file does not match with expected output"
    );

    block.content = "\\ *italic*\\".to_string();
    expected_html = format!("<p id='{}'>\\ <i>italic</i>\\</p>", id);
    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap(),
        "Html file does not match with expected output"
    );

    block.content = "**\\*only bold\\***".to_string();
    expected_html = format!("<p id='{}'><b>\\*only bold\\*</b></p>", id);

    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap(),
        "Html file does not match with expected output"
    );
}
