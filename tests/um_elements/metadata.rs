use std::path::Path;

use super::super::middleend::ir_test_setup;
use clap::StructOpt;
use unimarkup_rs::{
    config::Config,
    frontend::{self},
    middleend,
    middleend::{IrError, MetadataIrLine},
    um_elements::{Metadata, MetadataKind},
    um_error::UmError,
};

#[test]
fn root_metadata_in_ir() -> Result<(), UmError> {
    let testfile = "tests/test_files/small_testfile.um";

    let mut connection = ir_test_setup::setup_test_ir();
    let mut cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", testfile]);

    frontend::run(&mut connection, &mut cfg)?;

    let expected_metadata = Metadata {
        file: Path::new(testfile).to_path_buf(),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    let ir_metadata: MetadataIrLine = expected_metadata.into();

    if let Ok(transaction) = connection.transaction() {
        let metadata_exists = middleend::entry_already_exists(&ir_metadata, &transaction);

        assert!(metadata_exists);
        return Ok(());
    }

    Err(UmError::Ir(IrError {
        tablename: "metadata".to_string(),
        column: "-".to_string(),
        message: "given metadata not found".to_string(),
    }))
}
