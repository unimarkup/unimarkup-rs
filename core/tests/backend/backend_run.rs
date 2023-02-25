use clap::StructOpt;
use unimarkup_core::{
    backend::{self, Render},
    config::Config,
    elements::{types::UnimarkupFile, HeadingBlock, HeadingLevel},
};
use unimarkup_inline::ParseUnimarkupInlines;

#[test]
fn test__backend_run__heading_block() {
    let block = HeadingBlock {
        id: "some-id".into(),
        level: HeadingLevel::Level1,
        content: "This is a heading".parse_unimarkup_inlines().collect(),
        attributes: "{}".into(),
        line_nr: 0,
    };

    let unimarkup_file = UnimarkupFile {
        blocks: vec![block.clone().into()],
        ..Default::default()
    };

    let cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", "in_file.um"]);

    #[allow(clippy::redundant_clone)]
    let mut out_path = cfg.um_file.clone();
    out_path.set_extension("html");

    let document = backend::run(unimarkup_file, cfg).unwrap();

    let html = document.html();

    let content = html.body();

    assert_eq!(block.render_html().expect("Block is checked"), content);
}
