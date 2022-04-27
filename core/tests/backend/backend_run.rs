use clap::StructOpt;
use unimarkup_core::{
    backend::{self, Render},
    config::Config,
    elements::{get_column_offset_from_level, HeadingBlock, HeadingLevel},
    middleend::{self, AsIrLines, ContentIrLine},
};
use unimarkup_inline::{parse_with_offset, Position};

use super::super::middleend::ir_test_setup;

#[test]
fn test__backend_run__heading_block() {
    let mut connection = ir_test_setup::setup_test_ir();

    let block = HeadingBlock {
        id: "some-id".into(),
        level: HeadingLevel::Level1,
        content: parse_with_offset(
            "This is a heading",
            Position {
                line: 0,
                column: get_column_offset_from_level(HeadingLevel::Level1),
            },
        )
        .unwrap(),
        attributes: "{}".into(),
        line_nr: 0,
    };

    let lines: Vec<ContentIrLine> = block.as_ir_lines();

    {
        let transaction = ir_test_setup::get_test_transaction(&mut connection);
        middleend::write_ir_lines(&lines, &transaction).unwrap();

        transaction.commit().unwrap();
    }

    let cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", "in_file.um"]);

    #[allow(clippy::redundant_clone)]
    let mut out_path = cfg.um_file.clone();
    out_path.set_extension("html");

    let document = backend::run(&mut connection, cfg).unwrap();

    let html = document.html();

    let content = html.body();

    assert_eq!(block.render_html().expect("Block is checked"), content);
}
