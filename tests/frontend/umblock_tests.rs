use std::path::Path;

use unimarkup_rs::{
    frontend::parser::{self},
    middleend::ContentIrLine,
    um_elements::types::{self, UnimarkupBlock},
    um_error::UmError,
};

#[test]
fn umblock_tests() -> Result<(), UmError> {
    let mut um_blocks =
        parser::parse_unimarkup(Path::new("tests/test_files/frontend/heading1.um"))?;
    //heading1.um
    loop_through_ir_lines(&um_blocks, heading1_expected_result());

    um_blocks = parser::parse_unimarkup(Path::new("tests/test_files/frontend/paragraph1.um"))?;

    //paragraph1.um
    loop_through_ir_lines(&um_blocks, paragraph1_expected_result());

    um_blocks = parser::parse_unimarkup(Path::new("tests/test_files/frontend/paragraph2.um"))?;
    //paragraph2.um
    loop_through_ir_lines(&um_blocks, paragraph2_expected_result());

    um_blocks = parser::parse_unimarkup(Path::new("tests/test_files/frontend/paragraph3.um"))?;
    //paragraph3.um
    loop_through_ir_lines(&um_blocks, paragraph3_expected_result());

    Ok(())
}

fn loop_through_ir_lines(
    um_blocks: &[Box<dyn UnimarkupBlock>],
    mut blocks_vector: Vec<ContentIrLine>,
) {
    for block in um_blocks {
        for ir_line in block.as_ir_lines() {
            ir_lines_assert_eq(
                ir_line,
                blocks_vector
                    .pop()
                    .expect("UnimarkupBlock from Vector that matches output"),
            );
        }
    }
}

fn ir_lines_assert_eq(line: ContentIrLine, heading_ir_line: ContentIrLine) {
    assert_eq!(line.id, heading_ir_line.id);
    assert_eq!(line.line_nr, heading_ir_line.line_nr);
    assert_eq!(line.um_type, heading_ir_line.um_type);
    assert_eq!(line.text, heading_ir_line.text);
    assert_eq!(line.fallback_text, heading_ir_line.fallback_text);
    assert_eq!(line.attributes, heading_ir_line.attributes);
    assert_eq!(
        line.fallback_attributes,
        heading_ir_line.fallback_attributes
    );
}

fn heading1_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "head1",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head1",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subhead-1",
        2,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead 1",
        "",
        "{}",
        "",
    ));
    blocks_vector.push(ContentIrLine::new(
        "subhead-1",
        3,
        format!("heading{delim}level{delim}3", delim = types::DELIMITER),
        "subhead 1",
        "",
        "{}",
        "",
    ));
    blocks_vector.reverse();
    blocks_vector
}

fn paragraph1_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "head1",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head1",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-3",
        3,
        "paragraph",
        "paragraph 1",
        "",
        "{}",
        "",
    ));

    blocks_vector.reverse();
    blocks_vector
}

fn paragraph2_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "multi-line-header",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "multi\nline header",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-4",
        4,
        "paragraph",
        "paragraph",
        "",
        "{}",
        "",
    ));

    blocks_vector.reverse();
    blocks_vector
}

fn paragraph3_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "head2",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head2",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-3",
        3,
        "paragraph",
        "paragraph1",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subhead2",
        5,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead2",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "paragraph-7",
        7,
        "paragraph",
        "paragraph2",
        "",
        "{}",
        "",
    ));

    blocks_vector.reverse();
    blocks_vector
}
