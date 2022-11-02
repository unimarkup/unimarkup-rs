use std::path::Path;

use super::super::middleend::test_setup;
use clap::StructOpt;
use unimarkup_core::{
    config::Config,
    frontend,
    metadata::{Metadata, MetadataKind},
    middleend,
    middleend::MetadataIrLine,
    security,
};

#[test]
fn test__ir_root__metadata_in_ir() {
    let testfile = "tests/test_files/small_testfile.um";

    let mut connection = test_setup::setup_test_ir();
    let mut cfg: Config = Config::parse_from(vec!["unimarkup", "--output-formats=html", testfile]);

    let input = std::fs::read_to_string(&cfg.um_file).unwrap();

    let result = frontend::run(&input, &mut connection, &mut cfg);

    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());

    let expected_metadata = Metadata {
        file: Path::new(testfile).to_path_buf(),
        contenthash: security::get_filehash(Path::new(testfile)).unwrap(),
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

#[test]
fn test__metadata__create_from_memory() {
    let testfile = "from_memory";
    let content = "some **unimarkup content**";

    let metadata = Metadata {
        file: Path::new(testfile).to_path_buf(),
        contenthash: security::get_contenthash(content),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    assert_eq!(
        metadata.file.to_str().unwrap(),
        testfile,
        "Creating metadata from memory content failed"
    );
}
