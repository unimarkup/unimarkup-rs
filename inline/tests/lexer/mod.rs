use std::panic;

use libtest_mimic::Trial;
use unimarkup_commons::test_runner::{self, snap_test_runner::SnapTestRunner};
use unimarkup_inline::Tokenize;

use crate::snapshot::Snapshot;

mod snapshot;

pub fn test_lexer_snapshots() -> Vec<Trial> {
    let tests_path = unimarkup_commons::crate_tests_path!();
    let test_cases = test_runner::collect_tests(
        tests_path.join("spec/markup"),
        tests_path.join("spec/snapshots/lexer"),
        "markup",
    );

    let mut test_runs = Vec::with_capacity(test_cases.len());
    for case in test_cases {
        let test_name = format!("{}::{}", module_path!(), case.test.name.as_str());

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

fn run_test_case(case: test_runner::test_file::TestCase) {
    let mut symbols = unimarkup_commons::scanner::scan_str(&case.test.input);
    symbols.pop(); // Remove EOI symbol for test cases

    let runner = SnapTestRunner::with_fn(&case.test.name, &symbols, |symbols| {
        Snapshot::snap((case.test.input.as_ref(), symbols.tokens()))
    })
    .with_info(format!(
        "Test '{}' from '{}'",
        case.test.name, case.file_path
    ));

    unimarkup_commons::run_snap_test!(runner, &case.out_path);
}
