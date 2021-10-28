use unimarkup_rs::{
    frontend::parser::CursorPos, middleend::ir::ParseForIr,
    um_elements::heading_block::HeadingBlock,
};

#[test]
fn parse_heading() {
    let input = "###### This is a heading which should fail

	"
    .lines()
    .collect::<Vec<&str>>();

    let cursor_pos = CursorPos { line: 0, symbol: 0 };

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    assert!(res.is_ok());
}
