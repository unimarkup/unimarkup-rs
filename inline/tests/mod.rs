mod parser;
mod snapshot;

use libtest_mimic::Arguments;
use parser::test_parser_snapshots;
pub(crate) use snapshot::*;

fn main() {
    let args = Arguments::from_args();
    let parser_tests = test_parser_snapshots();

    libtest_mimic::run(&args, parser_tests).exit();
}
