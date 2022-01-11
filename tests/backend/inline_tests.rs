use unimarkup_rs::{backend::Render, um_elements::ParagraphBlock, um_error::UmError};

#[test]

fn escaped_inline() -> Result<(), UmError> {
    let id = String::from("paragraph-id");
    let content = String::from("\\*23\\*3");

    let mut block = ParagraphBlock {
        id: id.clone(),
        content,
        attributes: "{}".into(),
        line_nr: 0,
    };

    let mut expected_html = format!("<p id='{}'>\\*23\\*3</p>", id);

    assert_eq!(expected_html, block.render_html()?);

    block.content = "\\ *italic*\\".to_string();
    expected_html = format!("<p id='{}'>\\ <i>italic</i>\\</p>", id);
    assert_eq!(expected_html, block.render_html()?);

    block.content = "**\\*only bold\\***".to_string();
    expected_html = format!("<p id='{}'><b>\\*only bold\\*</b></p>", id);
    assert_eq!(expected_html, block.render_html()?);

    Ok(())
}
