use unimarkup_commons::test_runner::{
    self, as_snapshot::AsSnapshot, snap_test_runner::SnapTestRunner, test_file,
};
use unimarkup_core::Unimarkup;

use crate::snapshot::Snapshot;

pub fn test_core_snapshots() -> Vec<libtest_mimic::Trial> {
    let tests_path = unimarkup_commons::crate_tests_path!();

    let test_cases = test_runner::collect_tests(
        tests_path.join("spec/markup"),
        tests_path.join("spec/snapshots/"),
        "markup",
    );

    let mut test_runs = Vec::with_capacity(test_cases.len());

    for case in test_cases {
        let spec_test_name = format!("{}::spec::{}", module_path!(), case.test.name.as_str());
        let snap_test_name = format!("{}::snap::{}", module_path!(), case.test.name.as_str());

        let cloned_case = case.clone();
        let spec_test_run = move || {
            std::panic::catch_unwind(|| run_spec_test(cloned_case)).map_err(|err| {
                let panic_msg = err
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Panic message not available");

                format!("Test case panicked: {}", panic_msg).into()
            })
        };

        let snap_test_run = move || {
            std::panic::catch_unwind(|| run_snap_test(case)).map_err(|err| {
                let panic_msg = err
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Panic message not available");

                format!("Test case panicked: {}", panic_msg).into()
            })
        };

        test_runs.push(libtest_mimic::Trial::test(spec_test_name, spec_test_run));
        test_runs.push(libtest_mimic::Trial::test(snap_test_name, snap_test_run));
    }

    test_runs
}

fn run_spec_test(case: test_runner::test_file::TestCase) {
    test_runner::spec_test::assert_um_spec(
        &case.file_name,
        &case.test,
        unimarkup_commons::config::Config::default(),
        |test: &test_file::Test, cfg| {
            let input = test.input.trim_end();
            let um = Unimarkup::parse(input, cfg);
            test_file::TestOutputs {
                html: Some(um.render_html().unwrap().to_string()),
                um: Some(test.input.clone()),
            }
        },
    );
}

fn run_snap_test(case: test_runner::test_file::TestCase) {
    let tokens = unimarkup_commons::lexer::token::lex_str(&case.test.input);

    let mut snap_runner = SnapTestRunner::with_fn::<_, _>(&case.test.name, &tokens, |_input| {
        let um = unimarkup_core::parser::parse_unimarkup(
            &case.test.input,
            unimarkup_commons::config::Config::default(),
        );

        Snapshot(um.blocks).as_snapshot()
    })
    .with_info(format!(
        "Test '{}' from: {}",
        case.test.name, case.file_path
    ));

    if let Some(ref description) = case.test.description {
        snap_runner = snap_runner.with_description(description);
    }

    unimarkup_commons::run_snap_test!(snap_runner, &case.out_path);
}
