use unimarkup_core::elements::types;
use unimarkup_core::frontend::parser;
use unimarkup_core::middleend::ContentIrLine;

use super::tests_helper::*;

#[test]
fn test__parse__valid_heading_with_subheading() {
    let mut config = get_config("tests/test_files/frontend/heading1.um");
    let input = get_file_content(&config.um_file);
    //heading1.um
    let um_blocks = parser::parse_unimarkup(&input, &mut config);
    assert!(um_blocks.is_ok(), "Cause: {:?}", um_blocks.unwrap_err());
    loop_through_ir_lines(&um_blocks.unwrap(), heading1_expected_result());
}

#[test]
fn test__parse__valid_nested_headings_with_blank_lines() {
    let mut config = get_config("tests/test_files/frontend/heading_line_number.um");
    let input = get_file_content(&config.um_file);
    //heading_line_number.um
    let um_blocks = parser::parse_unimarkup(&input, &mut config);
    assert!(um_blocks.is_ok(), "Cause: {:?}", um_blocks.unwrap_err());
    loop_through_ir_lines(&um_blocks.unwrap(), heading_line_number_expected_result());
}

#[test]
fn test__parse__valid_multi_line_heading() {
    let mut config = get_config("tests/test_files/frontend/multiline_headings.um");
    let input = get_file_content(&config.um_file);
    //multiline_headings.um
    let um_blocks = parser::parse_unimarkup(&input, &mut config);
    assert!(um_blocks.is_ok(), "Cause: {:?}", um_blocks.unwrap_err());
    loop_through_ir_lines(&um_blocks.unwrap(), multiline_headings_expected_result());
}

pub fn heading1_expected_result() -> Vec<ContentIrLine> {
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

pub fn heading_line_number_expected_result() -> Vec<ContentIrLine> {
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
        "subhead-11",
        2,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead 11",
        "",
        "{}",
        "",
    ));
    blocks_vector.push(ContentIrLine::new(
        "head2",
        4,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head2",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subhead-21",
        5,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead 21",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "head3",
        8,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head3",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subhead-31",
        9,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead 31",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subsubhead-311",
        10,
        format!("heading{delim}level{delim}3", delim = types::DELIMITER),
        "subsubhead 311",
        "",
        "{}",
        "",
    ));
    blocks_vector.reverse();
    blocks_vector
}

pub fn multiline_headings_expected_result() -> Vec<ContentIrLine> {
    let mut blocks_vector: Vec<ContentIrLine> = Vec::new();
    blocks_vector.push(ContentIrLine::new(
        "head1-multiline",
        1,
        format!("heading{delim}level{delim}1", delim = types::DELIMITER),
        "head1\nmultiline",
        "",
        "{}",
        "",
    ));

    blocks_vector.push(ContentIrLine::new(
        "subhead2-multiline",
        3,
        format!("heading{delim}level{delim}2", delim = types::DELIMITER),
        "subhead2\nmultiline",
        "",
        "{}",
        "",
    ));
    blocks_vector.push(ContentIrLine::new(
        "paragraph-6",
        6,
        "paragraph",
        "paragraph 2",
        "",
        "{}",
        "",
    ));
    blocks_vector.reverse();
    blocks_vector
}
