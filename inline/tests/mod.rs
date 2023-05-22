use std::path::PathBuf;

mod lexer;

pub fn snap_path() -> PathBuf {
    let curr = std::env!("CARGO_MANIFEST_DIR");

    let mut buf = PathBuf::from(curr);
    buf.push("tests/spec/snapshots/");

    buf.canonicalize().unwrap()
}
