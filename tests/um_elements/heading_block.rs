use unimarkup_rs::{
    frontend::parser::CursorPos,
    middleend::ir::ParseForIr,
    um_elements::heading_block::{HeadingBlock, HeadingLevel},
};

#[test]
fn parse_heading() {
    let lowest_level = HeadingLevel::Level1 as usize;
    let highest_level = HeadingLevel::Level6 as usize;

    for level in lowest_level..=highest_level {
        let heading_prefix = "#".repeat(level);
        let heading_content = "This is a heading";

        let heading = heading_prefix + " " + heading_content + "\n   \n";

        let cursor_pos = CursorPos { line: 0, symbol: 0 };
        let input: Vec<&str> = heading.lines().collect();

        let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

        assert!(res.is_ok());

        if let Ok((block, _)) = res {
            assert!(block.lines.len() == 1);

            let line = block.lines.get(0).expect("Exactly one line expected");

            assert!(line.text == "This is a heading");

            let mut heading_type: String = String::from("heading_");
            heading_type.push_str(&HeadingLevel::from(level).to_string());

            assert!(line.block_type == heading_type);

            println!("\n\nParsed heading block: \n{:#?}\n", block);
        }
    }
}

#[test]
fn parse_heading_no_hashes() {
    let heading_content = "Heading without '#' symbols

    ";

    let cursor_pos = CursorPos { line: 0, symbol: 0 };
    let input: Vec<&str> = heading_content.lines().collect();

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    assert!(res.is_err());

    let error = res.unwrap_err();
    assert!(error.message == "Unexpected symbol found!");
}

#[test]
fn parse_heading_no_whitespace() {
    let heading_content = "######Heading '#' symbols not followed by a whitespace

    ";

    let cursor_pos = CursorPos { line: 0, symbol: 0 };
    let input: Vec<&str> = heading_content.lines().collect();

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    assert!(res.is_err());

    let error = res.unwrap_err();
    assert!(error.message == "Unexpected symbol found!");

    let heading_content = "#######Heading '#' symbols not followed by a whitespace

    ";

    let cursor_pos = CursorPos { line: 0, symbol: 0 };
    let input: Vec<&str> = heading_content.lines().collect();

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    assert!(res.is_err());

    let error = res.unwrap_err();
    assert!(error.message == "Unexpected symbol found!");

    println!("{}", error);
}

#[test]
fn parse_heading_too_many_symbols() {
    let heading_content = "####### Heading too many '#' symbols

    ";

    let cursor_pos = CursorPos { line: 0, symbol: 0 };
    let input: Vec<&str> = heading_content.lines().collect();

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    assert!(res.is_err());

    let error = res.unwrap_err();
    assert!(error.message == "Invalid number of '#' symbols.");

    println!("{}", error);
}

#[test]
fn parse_heading_empty() {
    let heading_content = "

    ";

    let cursor_pos = CursorPos { line: 0, symbol: 0 };
    let input: Vec<&str> = heading_content.lines().collect();

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    assert!(res.is_err());

    let error = res.unwrap_err();
    assert!(
        error.message
            == "Invalid heading syntax. \n".to_owned()
                + "Headings are defined as 1 to 6 '#' symbols, \n"
                + "followed by whitespace and Heading content."
    );

    println!("{}", error);
}
