// mod lexer;
mod parser;
mod snapshot;

// use lexer::test_lexer_snapshots;
use libtest_mimic::Arguments;
use parser::test_parser_snapshots;
pub(crate) use snapshot::*;

fn main() {
    let args = Arguments::from_args();
    // let lexer_tests = test_lexer_snapshots();
    let parser_tests = test_parser_snapshots();

    // let mut tests = lexer_tests;
    // tests.append(&mut parser_tests);

    libtest_mimic::run(&args, parser_tests).exit();
}
