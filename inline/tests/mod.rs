use std::path::PathBuf;

mod lexer;
mod parser;
mod snapshot;

use lexer::test_lexer_snapshots;
use libtest_mimic::Arguments;
use parser::test_parser_snapshots;
pub(crate) use snapshot::*;

fn main() {
    let args = Arguments::from_args();
    let lexer_tests = test_lexer_snapshots();
    let mut parser_tests = test_parser_snapshots();

    let mut tests = lexer_tests;
    tests.append(&mut parser_tests);

    libtest_mimic::run(&args, tests).exit();
}

/// Returns the absolute path to the integration `tests` folder.
pub fn tests_path() -> PathBuf {
    let curr = std::env!("CARGO_MANIFEST_DIR");

    let mut buf = PathBuf::from(curr);
    buf.push("tests");

    buf
}
