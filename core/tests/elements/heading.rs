use unimarkup_core::elements::atomic::{Heading, HeadingLevel, Paragraph};
use unimarkup_core::elements::Blocks;
use unimarkup_core::parser;
use unimarkup_inline::ParseUnimarkupInlines;

use crate::assert_blocks_match;

use super::tests_helper::*;

#[test]
fn test__parse__valid_heading_with_subheading() {
    let mut config = get_config("tests/test_files/frontend/heading1.um");
    let input = get_file_content(&config.um_file);

    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing heading1.um should pass.");

    assert_blocks_match!(um_blocks, heading1_expected_result());
}

#[test]
fn test__parse__valid_nested_headings_with_blank_lines() {
    let mut config = get_config("tests/test_files/frontend/heading_line_number.um");
    let input = get_file_content(&config.um_file);

    let um_blocks = parser::parse_unimarkup(&input, &mut config)
        .expect("Parsing heading_line_number.um should pass.");

    assert_blocks_match!(um_blocks, heading_line_number_expected_result());
}

#[test]
fn test__parse__valid_multi_line_heading() {
    let mut config = get_config("tests/test_files/frontend/multiline_headings.um");
    let input = get_file_content(&config.um_file);

    let um_blocks = parser::parse_unimarkup(&input, &mut config)
        .expect("Parsing multiline_headings.um should pass");

    assert_blocks_match!(um_blocks, multiline_headings_expected_result());
}

pub fn heading1_expected_result() -> Blocks {
    let heading_1 = Heading {
        id: String::from("head1"),
        level: HeadingLevel::Level1,
        content: "head1".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 1,
    };

    let subheading_1 = Heading {
        id: String::from("subhead-1"),
        level: HeadingLevel::Level2,
        content: "subhead 1".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 2,
    };

    [heading_1, subheading_1]
        .into_iter()
        .map(From::from)
        .collect()
}

pub fn heading_line_number_expected_result() -> Blocks {
    let mut blocks = Vec::with_capacity(7);

    let head1 = Heading {
        id: "head1".into(),
        level: HeadingLevel::Level1,
        content: "head1".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 1,
    };

    blocks.push(head1);

    let subhead_11 = Heading {
        id: "subhead-11".into(),
        level: HeadingLevel::Level2,
        content: "subhead 11".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 2,
    };

    blocks.push(subhead_11);

    let head2 = Heading {
        id: "head2".into(),
        level: HeadingLevel::Level1,
        content: "head2".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 4,
    };

    blocks.push(head2);

    let subhead_21 = Heading {
        id: "subhead-21".into(),
        level: HeadingLevel::Level2,
        content: "subhead 21".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 5,
    };

    blocks.push(subhead_21);

    let head_3 = Heading {
        id: "head3".into(),
        level: HeadingLevel::Level1,
        content: "head3".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 8,
    };

    blocks.push(head_3);

    let subhead_31 = Heading {
        id: "subhead-31".into(),
        level: HeadingLevel::Level2,
        content: "subhead 31".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 9,
    };

    blocks.push(subhead_31);

    let subsubhead_311 = Heading {
        id: "subsubhead-311".into(),
        level: HeadingLevel::Level3,
        content: "subsubhead 311".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 10,
    };

    blocks.push(subsubhead_311);

    blocks.into_iter().map(From::from).collect()
}

pub fn multiline_headings_expected_result() -> Blocks {
    let mut blocks: Blocks = Vec::new();

    let block = Heading {
        id: "head1-multiline".into(),
        level: HeadingLevel::Level1,
        content: "head1\nmultiline".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = Heading {
        id: "subhead2-multiline".into(),
        level: HeadingLevel::Level2,
        content: "subhead2\nmultiline".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 3,
    };

    blocks.push(block.into());

    let block = Paragraph {
        id: "paragraph-6".into(),
        content: "paragraph 2".parse_unimarkup_inlines().collect(),
        attributes: None,
        line_nr: 6,
    };

    blocks.push(block.into());
    blocks
}
