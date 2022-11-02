use unimarkup_core::elements::ParagraphBlock;
use unimarkup_inline::ParseUnimarkupInlines;
use unimarkup_render::render::Render;

#[test]
fn test__render_html__valid_escaped_inline() {
    let id = String::from("paragraph-id");
    let content = "\\*23\\*3".parse_unimarkup_inlines().collect();

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
        result.unwrap().body,
        "Html file does not match with expected output"
    );

    block.content = "\\ *italic*\\".parse_unimarkup_inlines().collect();
    expected_html = format!("<p id='{}'> <em>italic</em></p>", id);

    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap().body,
        "Html file does not match with expected output"
    );

    block.content = "**\\*only bold\\***".parse_unimarkup_inlines().collect();
    expected_html = format!("<p id='{}'><strong>*only bold*</strong></p>", id);

    let result = block.render_html();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    assert_eq!(
        expected_html,
        result.unwrap().body,
        "Html file does not match with expected output"
    );
}
