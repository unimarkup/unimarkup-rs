use std::{fmt::Write, panic};

use libtest_mimic::Trial;
use unimarkup_commons::test_runner::{
    self, as_snapshot::AsSnapshot, snap_test_runner::SnapTestRunner,
};

use crate::snapshot::Snapshot;

mod snapshot;

pub(crate) fn collect_snapshot_tests() -> Vec<Trial> {
    let tests_path = unimarkup_commons::crate_tests_path!();
    let test_cases = test_runner::collect_tests(
        tests_path.join("spec/markup"),
        tests_path.join("spec/snapshots/lexer"),
        "markup",
    );

    let mut test_runs = Vec::with_capacity(test_cases.len());

    for case in test_cases {
        let snap_test_name = format!("{}::snap::{}", module_path!(), case.test.name.as_str());

        let snap_test_run = move || {
            panic::catch_unwind(|| run_snap_test(case)).map_err(|err| {
                let panic_msg = err
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Panic message not available");

                format!("Test case panicked: {}", panic_msg).into()
            })
        };

        test_runs.push(Trial::test(snap_test_name, snap_test_run));
    }

    test_runs
}

fn run_snap_test(case: test_runner::test_file::TestCase) {
    let runner = SnapTestRunner::with_fn(&case.test.name, &case.test.input, |input_str| {
        let token_stream = unimarkup_frontend::lexer::TokenStream::tokenize(input_str);

        let token_snaps = token_stream
            .map(Snapshot)
            .fold(String::new(), |mut agg, snap| {
                let _ = writeln!(&mut agg, "{}", snap.as_snapshot());
                agg
            });
        format!("{input_str}\n{token_snaps}")
    })
    .with_info(format!(
        "Test '{}' from '{}'",
        case.test.name, case.file_path
    ));

    unimarkup_commons::run_snap_test!(runner, &case.out_path);
}
