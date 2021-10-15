use unicode_segmentation::UnicodeSegmentation;
use unimarkup_rs::frontend::{blocks::heading_block::HeadingLevel, parser};

#[test]
fn parse_heading_level1() {
    let input = "# Heading level 1"
        .split_word_bounds()
        .collect::<Vec<&str>>();

    let output_block = parser::parse_heading(&input, &mut 0);

    assert_eq!(output_block.level, HeadingLevel::Level1);
    assert_eq!(output_block.content, String::from("Heading level 1"));
}

#[test]
fn parse_heading_level2() {
    let input = "## Heading level 2"
        .split_word_bounds()
        .collect::<Vec<&str>>();

    println!("{:?}", input);

    let output_block = parser::parse_heading(&input, &mut 0);

    assert_eq!(output_block.level, HeadingLevel::Level2);
    assert_eq!(output_block.content, String::from("Heading level 2"));
}

#[test]
fn parse_heading_level3() {
    let input = "### Heading level 3"
        .split_word_bounds()
        .collect::<Vec<&str>>();

    let output_block = parser::parse_heading(&input, &mut 0);

    assert_eq!(output_block.level, HeadingLevel::Level3);
    assert_eq!(output_block.content, String::from("Heading level 3"));
}
