use std::path::Path;

use super::super::middleend::ir_test_setup;
use clap::StructOpt;
use unimarkup_core::{
    config::Config,
    elements::{Metadata, MetadataKind},
    error::UmError,
    frontend, middleend,
    middleend::{IrError, MetadataIrLine},
};

#[test]
fn test_ir_root_metadata_in_ir() {
    let testfile = "tests/test_files/small_testfile.um";

    let mut connection = ir_test_setup::setup_test_ir();
    let mut cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", testfile]);

    let input = std::fs::read_to_string(&cfg.um_file).unwrap();

    let result = frontend::run(&input, &mut connection, &mut cfg);

    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());

    let expected_metadata = Metadata {
        file: Path::new(testfile).to_path_buf(),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    let ir_metadata: MetadataIrLine = expected_metadata.into();

    let transaction = connection.transaction();

    assert!(
        transaction.is_ok(),
        "Cause: {:?}",
        UmError::Ir(IrError {
            tablename: "metadata".to_string(),
            column: "-".to_string(),
            message: "given metadata not found".to_string(),
        })
    );

    let metadata_exists = middleend::entry_already_exists(&ir_metadata, &transaction.unwrap());
    assert!(metadata_exists, "Metadata does not exist");
}
