use std::path::PathBuf;

use unimarkup_core::{commons::config::Config, Unimarkup};

fn compile_um(config: Config) -> Option<Unimarkup> {
    let source = std::fs::read_to_string(&config.input).ok()?;

    Some(Unimarkup::parse(&source, config))
}

#[test]
fn umi_loop() {
    let mut config = Config::default();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    path.push("tests/test_files/supported.um");
    config.input = path;

    let um = compile_um(config).unwrap();
    let mut umi = um.render_umi().unwrap();
    let workbook = umi.create_workbook();

    let looped_doc = workbook.create_um().map_err(|_| panic!()).unwrap();

    assert_eq!(
        looped_doc.blocks.len(),
        um.get_document().blocks.len(),
        "Parsed UMI file differs from original UM."
    );
}
