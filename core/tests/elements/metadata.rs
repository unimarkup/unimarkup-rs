use std::path::Path;

use unimarkup_core::elements::{Metadata, MetadataKind};
use unimarkup_core::security;

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
