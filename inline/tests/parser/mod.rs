use crate::snapshot::Snapshot;
use unimarkup_commons::test_runner::{snap_test_runner::SnapTestRunner, test_file::TestFile};
use unimarkup_inline::ParseInlines;

mod snapshot;

#[test]
fn test_parser_snapshots() {
    let mut markups_path = crate::tests_path();
    markups_path.push("spec/markup");

    let entries = crate::collect_entries(markups_path, "yml").unwrap();

    for entry in entries {
        let path = entry.path();
        let input = std::fs::read_to_string(&path).unwrap();

        let test_file: TestFile = serde_yaml::from_str(&input).unwrap();

        for test in test_file.tests {
            let input = test.input;
            let out_path = crate::gen_snap_path("spec/snapshots/parser", &path);

            let file_name = path.file_name().and_then(|file| file.to_str()).unwrap();

            let runner = SnapTestRunner::with_fn(&test.name, input.as_str(), |symbols| {
                let rest = &[];
                let inlines: Vec<_> = symbols.parse_inlines().collect();
                let snapshot = Snapshot::snap(&inlines[..]);
                (snapshot, rest)
            })
            .with_info(format!("Test '{}' from: '{}'", test.name, file_name));

            unimarkup_commons::run_snap_test!(runner, &out_path);
        }
    }
}
