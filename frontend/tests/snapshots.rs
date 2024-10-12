mod lexer;
mod snapshot;

use libtest_mimic::Arguments;

// pub(crate) use snapshot::*;

fn main() {
    let args = Arguments::from_args();
    let lexer_tests = lexer::collect_snapshot_tests();

    libtest_mimic::run(&args, lexer_tests).exit();
}

