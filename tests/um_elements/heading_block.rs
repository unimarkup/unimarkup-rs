use unimarkup_rs::{
    frontend::{ir::ParseForIr, parser::CursorPos},
    um_elements::heading_block::HeadingBlock,
};

#[test]
fn parse_heading() {
    let input = "####### This is a heading which should fail

	"
    .lines()
    .collect::<Vec<&str>>();

    let cursor_pos = CursorPos { line: 0, symbol: 0 };

    let res = HeadingBlock::parse_for_ir(&input, &cursor_pos);

    match &res {
        Ok(_) => todo!(),
        Err(error) => println!("{}", error),
    }

    assert!(res.is_ok());
}
