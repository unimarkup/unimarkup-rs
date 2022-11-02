use clap::StructOpt;
use unimarkup_core::{
    backend,
    config::Config,
    elements::atomic::{Heading, HeadingLevel},
    middleend::{self, AsIrLines, ContentIrLine},
};
use unimarkup_inline::ParseUnimarkupInlines;
use unimarkup_render::render::Render;

use super::super::middleend::test_setup;

#[test]
fn test__backend_run__heading_block() {
    let mut connection = test_setup::setup_test_ir();

    let block = Heading {
        id: "some-id".into(),
        level: HeadingLevel::Level1,
        content: "This is a heading".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 0,
    };

    let lines: Vec<ContentIrLine> = block.as_ir_lines();

    {
        let transaction = test_setup::get_test_transaction(&mut connection);
        middleend::write_ir_lines(&lines, &transaction).unwrap();

        transaction.commit().unwrap();
    }

    let cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", "in_file.um"]);

    #[allow(clippy::redundant_clone)]
    let mut out_path = cfg.um_file.clone();
    out_path.set_extension("html");

    let document = backend::run(&mut connection, cfg).unwrap();

    let html = document.html();

    let content = html.body;

    assert_eq!(block.render_html().expect("Block is checked").body, content);
}
