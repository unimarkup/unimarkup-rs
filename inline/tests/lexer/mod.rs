use unimarkup_commons::test_runner::test_file::TestFile;

mod snapshot;

macro_rules! test_lexing {
    (
        test_name: $test_name:expr,
        test_file: $test_file:expr,
        input: $input:expr, 
        out_path: $out_path:expr
    ) => {
        use unimarkup_commons::test_runner::{
            as_snapshot::AsSnapshot, snap_test_runner::SnapTestRunner,
        };
        use unimarkup_inline::Tokenize;
        use $crate::lexer::snapshot::Snapshot;

        let runner = SnapTestRunner::with_fn(&$test_name, $input, |symbols| {
            let rest = &[];
            let snapshot = Snapshot(($input, symbols.tokens())).as_snapshot();
            (snapshot, rest)
        })
        .with_info(format!(
            "Test '{}' from: '{}'",
            $test_name,
            $test_file,
        ));

        unimarkup_commons::run_snap_test!(runner, $out_path);
    };
}

#[test]
fn test_lexer_snapshots() {
    let mut markups_path = crate::tests_path();
    markups_path.push("spec/markup");

    let entries = crate::collect_entries(markups_path, "yml").unwrap();

    for entry in entries {
        let path = entry.path();
        let input = std::fs::read_to_string(&path).unwrap();

        let test_file: TestFile = serde_yaml::from_str(&input).unwrap();

        for test in test_file.tests {
            let input = test.input;
            let out_path = crate::gen_snap_path("spec/snapshots/lexer", &path);

            let file_name = path.file_name().and_then(|file| file.to_str()).unwrap();

            test_lexing!{
                test_name: test.name, 
                test_file: file_name,
                input: input.as_str(), 
                out_path: &out_path
            }
        }
    }
}
