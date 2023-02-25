use unimarkup_core::elements::types::UnimarkupBlocks;
use unimarkup_core::elements::{HeadingBlock, HeadingLevel, ParagraphBlock};
use unimarkup_core::frontend::parser;
use unimarkup_inline::ParseUnimarkupInlines;

use super::tests_helper::*;

#[test]
fn test__parse__valid_paragraph_with_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph1.um");
    let input = get_file_content(&config.um_file);
    //paragraph1.um
    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing paragraph1.um should pass.");

    assert_blocks_match(um_blocks, paragraph1_expected_result());
}

#[test]
fn test__parse__valid_paragraph_with_multi_line_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph2.um");
    let input = get_file_content(&config.um_file);
    //paragraph2.um
    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing paragraph2.um should pass.");
    assert_blocks_match(um_blocks, paragraph2_expected_result());
}

#[test]
fn test__parse__valid_paragraphs_with_sub_heading() {
    let mut config = get_config("tests/test_files/frontend/paragraph3.um");
    let input = get_file_content(&config.um_file);
    //paragraph3.um
    let um_blocks =
        parser::parse_unimarkup(&input, &mut config).expect("Parsing paragraph3.um should pass.");

    assert_blocks_match(um_blocks, paragraph3_expected_result());
}

pub fn paragraph1_expected_result() -> UnimarkupBlocks {
    let mut blocks: UnimarkupBlocks = Vec::with_capacity(2);

    let block = HeadingBlock {
        id: "head1".into(),
        level: HeadingLevel::Level1,
        content: "head1".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = ParagraphBlock {
        id: "paragraph-3".into(),
        content: "paragraph 1".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 3,
    };

    blocks.push(block.into());

    blocks
}

pub fn paragraph2_expected_result() -> UnimarkupBlocks {
    let mut blocks: UnimarkupBlocks = Vec::with_capacity(2);

    let block = HeadingBlock {
        id: "multi-line-header".into(),
        level: HeadingLevel::Level1,
        content: "multi\nline header".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = ParagraphBlock {
        id: "paragraph-4".into(),
        content: "paragraph".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 4,
    };

    blocks.push(block.into());

    blocks
}

pub fn paragraph3_expected_result() -> UnimarkupBlocks {
    let mut blocks: UnimarkupBlocks = Vec::new();
    let block = HeadingBlock {
        id: "head2".into(),
        level: HeadingLevel::Level1,
        content: "head2".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 1,
    };

    blocks.push(block.into());

    let block = ParagraphBlock {
        id: "paragraph-3".into(),
        content: "paragraph1".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 3,
    };

    blocks.push(block.into());

    let block = HeadingBlock {
        id: "subhead2".into(),
        level: HeadingLevel::Level2,
        content: "subhead2".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 5,
    };

    blocks.push(block.into());

    let block = ParagraphBlock {
        id: "paragraph-7".into(),
        content: "paragraph2".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 7,
    };

    blocks.push(block.into());

    blocks
}
