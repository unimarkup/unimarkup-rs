use unimarkup_core::elements::types;
use unimarkup_core::frontend::parser;
use unimarkup_core::middleend::ContentIrLine;

use super::tests_helper::*;

#[test]
fn test__parse__valid_paragraph_with_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph1.um");
    let input = get_file_content(&config.um_file);
    //paragraph1.um
    let um_blocks = parser::parse_unimarkup(&input, &mut config);
    assert!(um_blocks.is_ok(), "Cause: {:?}", um_blocks.unwrap_err());
    loop_through_ir_lines(&um_blocks.unwrap(), paragraph1_expected_result());
}

#[test]
fn test__parse__valid_paragraph_with_multi_line_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph2.um");
    let input = get_file_content(&config.um_file);
    //paragraph2.um
    let um_blocks = parser::parse_unimarkup(&input, &mut config);
    assert!(um_blocks.is_ok(), "Cause: {:?}", um_blocks.unwrap_err());
    loop_through_ir_lines(&um_blocks.unwrap(), paragraph2_expected_result());
}

#[test]
fn test__parse__valid_paragraphs_with_sub_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph3.um");
    let input = get_file_content(&config.um_file);
    //paragraph3.um
    let um_blocks = parser::parse_unimarkup(&input, &mut config);
    assert!(um_blocks.is_ok(), "Cause: {:?}", um_blocks.unwrap_err());
    loop_through_ir_lines(&um_blocks.unwrap(), paragraph3_expected_result());
}

pub fn paragraph1_expected_result() -> Vec<ContentIrLine> {
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

pub fn paragraph2_expected_result() -> Vec<ContentIrLine> {
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

pub fn paragraph3_expected_result() -> Vec<ContentIrLine> {
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
