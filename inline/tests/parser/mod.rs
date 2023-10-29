use std::panic;

use crate::snapshot::Snapshot;
use libtest_mimic::Trial;
use unimarkup_commons::{
    parsing::Context,
    test_runner::{self, snap_test_runner::SnapTestRunner},
};

mod snapshot;

pub fn test_parser_snapshots() -> Vec<Trial> {
    let tests_path = unimarkup_commons::crate_tests_path!();
    let test_cases = test_runner::collect_tests(
        tests_path.join("spec/markup"),
        tests_path.join("spec/snapshots/parser"),
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
    let symbols = unimarkup_commons::lexer::scan_str(&case.test.input);

    let runner = SnapTestRunner::with_fn(&case.test.name, &symbols, |symbols| {
        let inlines: Vec<_> =
            unimarkup_inline::inline_parser::parse_inlines(symbols, &mut Context::default());
        Snapshot::snap(&inlines[..])
    })
    .with_info(format!(
        "Test '{}' from '{}'",
        case.test.name, case.file_path
    ));

    unimarkup_commons::run_snap_test!(runner, &case.out_path);
}
