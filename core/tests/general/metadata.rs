use std::path::Path;

use unimarkup_parser::{
    metadata::{Metadata, MetadataKind},
    security,
};

pub fn create_metadata_from_memory() {
    let testfile = "from_memory";
    let content = "some **unimarkup content**";

    let metadata = Metadata {
        file: Path::new(testfile).to_path_buf(),
        contenthash: security::get_contenthash(content),
        preamble: None,
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    assert_eq!(
        metadata.file.to_str().unwrap(),
        testfile,
        "Creating metadata from memory content failed"
    );
}
