use std::panic;

use libtest_mimic::Trial;
use unimarkup_commons::test_runner::{self, snap_test_runner::SnapTestRunner};
use unimarkup_inline::Tokenize;

use crate::snapshot::Snapshot;

mod snapshot;

pub fn test_lexer_snapshots() -> Vec<Trial> {
    let test_cases = crate::prepare_test_cases("spec/markup", "spec/snapshots/lexer");
    let mut test_runs = Vec::with_capacity(test_cases.len());
    for case in test_cases {
        let test_name = format!("lexer:{}", case.name.as_str());

        let test_run =
            move || panic::catch_unwind(|| run_test_case(case)).map_err(|_| "Test panicked".into());

        test_runs.push(Trial::test(test_name, test_run));
    }

    test_runs
}

fn run_test_case(case: crate::TestCase) {
    let symbols = test_runner::scan_str(&case.input);
    let runner = SnapTestRunner::with_fn(&case.name, &symbols, |symbols| {
        let rest = &[];
        let snapshot = Snapshot::snap((case.input.as_ref(), symbols.tokens()));
        (snapshot, rest)
    })
    .with_info(format!("Test '{}' from '{}'", case.name, case.file_name));

    unimarkup_commons::run_snap_test!(runner, &case.out_path);
}
