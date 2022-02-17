use std::path::Path;

use super::super::middleend::ir_test_setup;
use clap::StructOpt;
use unimarkup_core::{
    config::Config,
    elements::{Metadata, MetadataKind},
    frontend, middleend,
    middleend::MetadataIrLine,
};

#[test]
fn root_metadata_in_ir() {
    let testfile = "tests/test_files/small_testfile.um";

    let mut connection = ir_test_setup::setup_test_ir();
    let mut cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", testfile]);

    let input = std::fs::read_to_string(&cfg.um_file).unwrap();

    frontend::run(&input, &mut connection, &mut cfg).unwrap();

    let expected_metadata = Metadata {
        file: Path::new(testfile).to_path_buf(),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    let ir_metadata: MetadataIrLine = expected_metadata.into();

    match connection.transaction() {
        Ok(transaction) => {
            let metadata_exists = middleend::entry_already_exists(&ir_metadata, &transaction);

            assert!(metadata_exists);
        }
        Err(_) => panic!("Failed creating database connection"),
    };
}
