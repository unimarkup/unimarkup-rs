use unimarkup_core::{backend::Render, elements::ParagraphBlock};

#[test]

fn escaped_inline() {
    let id = String::from("paragraph-id");
    let content = String::from("\\*23\\*3");

    let mut block = ParagraphBlock {
        id: id.clone(),
        content,
        attributes: "{}".into(),
        line_nr: 0,
    };

    let mut expected_html = format!("<p id='{}'>\\*23\\*3</p>", id);

    assert_eq!(expected_html, block.render_html().unwrap());

    block.content = "\\ *italic*\\".to_string();
    expected_html = format!("<p id='{}'>\\ <i>italic</i>\\</p>", id);
    assert_eq!(expected_html, block.render_html().unwrap());

    block.content = "**\\*only bold\\***".to_string();
    expected_html = format!("<p id='{}'><b>\\*only bold\\*</b></p>", id);
    assert_eq!(expected_html, block.render_html().unwrap());
}
