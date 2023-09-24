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
        let test_name = format!("{}::{}", module_path!(), case.name.as_str());

        let test_run = move || {
            panic::catch_unwind(|| run_test_case(case)).map_err(|err| {
                let panic_msg = err
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Panic message not available");

                format!("Test case panicked: {}", panic_msg).into()
            })
        };

        test_runs.push(Trial::test(test_name, test_run));
    }

    test_runs
}

fn run_test_case(case: crate::TestCase) {
    let mut symbols = test_runner::scan_str(&case.input);
    symbols.pop(); // Remove EOI symbol. TODO: handle EOI in lexer
    let runner = SnapTestRunner::with_fn(&case.name, &symbols, |symbols| {
        Snapshot::snap((case.input.as_ref(), symbols.tokens()))
    })
    .with_info(format!("Test '{}' from '{}'", case.name, case.file_name));

    unimarkup_commons::run_snap_test!(runner, &case.out_path);
}
