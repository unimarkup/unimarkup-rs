use unimarkup_commons::test_runner::{self, test_file};
use unimarkup_core::Unimarkup;

pub fn test_block_snapshots() -> Vec<libtest_mimic::Trial> {
    let tests_path = unimarkup_commons::crate_tests_path!();

    let test_cases = test_runner::collect_tests(
        tests_path.join("spec/markup"),
        tests_path.join("spec/snapshots/"),
        "markup",
    );

    let mut test_runs = Vec::with_capacity(test_cases.len());

    for case in test_cases {
        let test_name = format!("{}::{}", module_path!(), case.test.name.as_str());

        let test_run = move || {
            std::panic::catch_unwind(|| run_test_case(case)).map_err(|err| {
                let panic_msg = err
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Panic message not available");

                format!("Test case panicked: {}", panic_msg).into()
            })
        };

        test_runs.push(libtest_mimic::Trial::test(test_name, test_run));
    }

    test_runs
}

fn run_test_case(case: test_runner::test_file::TestCase) {
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
