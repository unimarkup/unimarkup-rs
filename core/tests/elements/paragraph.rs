use unimarkup_core::elements::atomic::{Heading, HeadingLevel, Paragraph};
use unimarkup_core::elements::Blocks;
use unimarkup_core::parser;
use unimarkup_inline::ParseInlines;

use crate::assert_blocks_match;

use super::tests_helper::*;

#[test]
fn test__parse__valid_paragraph_with_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph1.um");
    let input = get_file_content(&config.um_file);

    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing paragraph1.um should pass.");

    assert_blocks_match!(um_blocks, paragraph1_expected_result());
}

#[test]
fn test__parse__valid_paragraph_with_multi_line_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph2.um");
    let input = get_file_content(&config.um_file);

    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing paragraph2.um should pass.");
    assert_blocks_match!(um_blocks, paragraph2_expected_result());
}

#[test]
fn test__parse__valid_paragraphs_with_sub_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph3.um");
    let input = get_file_content(&config.um_file);

    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing paragraph3.um should pass.");

    assert_blocks_match!(um_blocks, paragraph3_expected_result());
}

pub fn paragraph1_expected_result() -> Blocks {
    let mut blocks: Blocks = Vec::with_capacity(2);

    let block = Heading {
        id: String::default(),
        level: HeadingLevel::Level1,
        content: " head1".parse_inlines(Some(unimarkup_inline::Position { line: 1, column: 2 })).collect(),
        attributes: None,
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = Paragraph {
        id: String::from("paragraph-3"),
        content: "paragraph 1".parse_inlines(Some(unimarkup_inline::Position { line: 3, column: 1 })).collect(),
        attributes: None,
        line_nr: 3,
    };

    blocks.push(block.into());

    blocks
}

pub fn paragraph2_expected_result() -> Blocks {
    let mut blocks: Blocks = Vec::with_capacity(2);

    let block = Heading {
        id: String::default(),
        level: HeadingLevel::Level1,
        content: " multi\nline header".parse_inlines(Some(unimarkup_inline::Position { line: 1, column: 2 })).collect(),
        attributes: None,
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = Paragraph {
        id: "paragraph-4".into(),
        content: "paragraph".parse_inlines(Some(unimarkup_inline::Position { line: 4, column: 1 })).collect(),
        attributes: None,
        line_nr: 4,
    };

    blocks.push(block.into());

    blocks
}

pub fn paragraph3_expected_result() -> Blocks {
    let mut blocks: Blocks = Vec::new();
    let block = Heading {
        id: String::default(),
        level: HeadingLevel::Level1,
        content: " head2".parse_inlines(Some(unimarkup_inline::Position { line: 1, column: 2 })).collect(),
        attributes: None,
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = Paragraph {
        id: "paragraph-3".into(),
        content: "paragraph1\n".parse_inlines(Some(unimarkup_inline::Position { line: 3, column: 1 })).collect(),
        attributes: None,
        line_nr: 3,
    };

    blocks.push(block.into());

    let block = Heading {
        id: String::default(),
        level: HeadingLevel::Level2,
        content: " subhead2".parse_inlines(Some(unimarkup_inline::Position { line: 5, column: 3 })).collect(),
        attributes: None,
        line_nr: 5,
    };

    blocks.push(block.into());

    let block = Paragraph {
        id: "paragraph-7".into(),
        content: "paragraph2".parse_inlines(Some(unimarkup_inline::Position { line: 7, column: 1 })).collect(),
        attributes: None,
        line_nr: 7,
    };

    blocks.push(block.into());

    blocks
}
