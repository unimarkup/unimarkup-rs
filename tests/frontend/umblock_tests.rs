use std::path::Path;

use unimarkup_rs::{
    frontend::parser::{self},
    middleend::ContentIrLine,
    um_elements::types::UnimarkupBlock,
    um_error::UmError,
};

use super::{
    heading_tests::{
        heading1_expected_result, heading_line_number_expected_result,
        multiline_headings_expected_result,
    },
    paragraph_tests::{
        paragraph1_expected_result, paragraph2_expected_result, paragraph3_expected_result,
    },
};

#[test]
fn umblock_tests() -> Result<(), UmError> {
    //heading1.um
    let mut um_blocks =
        parser::parse_unimarkup(Path::new("tests/test_files/frontend/heading1.um"))?;
    loop_through_ir_lines(&um_blocks, heading1_expected_result());

    //heading_line_number.um
    um_blocks = parser::parse_unimarkup(Path::new(
        "tests/test_files/frontend/heading_line_number.um",
    ))?;
    loop_through_ir_lines(&um_blocks, heading_line_number_expected_result());

    //multiline_headings.um
    um_blocks =
        parser::parse_unimarkup(Path::new("tests/test_files/frontend/multiline_headings.um"))?;
    loop_through_ir_lines(&um_blocks, multiline_headings_expected_result());

    //paragraph1.um
    um_blocks = parser::parse_unimarkup(Path::new("tests/test_files/frontend/paragraph1.um"))?;
    loop_through_ir_lines(&um_blocks, paragraph1_expected_result());

    //paragraph2.um
    um_blocks = parser::parse_unimarkup(Path::new("tests/test_files/frontend/paragraph2.um"))?;
    loop_through_ir_lines(&um_blocks, paragraph2_expected_result());

    //paragraph3.um
    um_blocks = parser::parse_unimarkup(Path::new("tests/test_files/frontend/paragraph3.um"))?;
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
    assert_eq!(line.fallback_text, heading_ir_line.fallback_text);
    assert_eq!(line.attributes, heading_ir_line.attributes);
    assert_eq!(
        line.fallback_attributes,
        heading_ir_line.fallback_attributes
    );
    ir_lines_text_eq(line.text, heading_ir_line.text);
}

fn ir_lines_text_eq(line_text: String, heading_ir_line_text: String) {
    let mut ir_lines = heading_ir_line_text.lines();

    for line in line_text.lines() {
        assert_eq!(line, ir_lines.next().expect("predefined text"));
    }
}
