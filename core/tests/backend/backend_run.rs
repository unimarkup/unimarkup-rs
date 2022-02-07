use clap::StructOpt;
use unimarkup_core::{
    backend::{self, Render},
    config::Config,
    elements::{HeadingBlock, HeadingLevel},
    error::UmError,
    middleend::{self, AsIrLines, ContentIrLine},
};

use super::super::middleend::ir_test_setup;

#[test]
fn run() -> Result<(), UmError> {
    let mut connection = ir_test_setup::setup_test_ir();

    let block = HeadingBlock {
        id: "some-id".into(),
        level: HeadingLevel::Level1,
        content: "This is a heading".into(),
        attributes: "{}".into(),
        line_nr: 0,
    };

    let lines: Vec<ContentIrLine> = block.as_ir_lines();

    {
        let transaction = ir_test_setup::get_test_transaction(&mut connection);
        middleend::write_ir_lines(&lines, &transaction)?;

        transaction.commit().unwrap();
    }

    let cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", "in_file.um"]);

    #[allow(clippy::redundant_clone)]
    let mut out_path = cfg.um_file.clone();
    out_path.set_extension("html");

    let document = backend::run(&mut connection, cfg)?;

    let html = document.html();

    let content = html.body();

    assert_eq!(block.render_html().expect("Block is checked"), content);

    Ok(())
}
