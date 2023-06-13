use crate::Snapshot;
use unimarkup_commons::test_runner::{self, snap_test_runner::SnapTestRunner};
use unimarkup_inline::Tokenize;

mod snapshot;

#[test]
fn test_lexer_snapshots() {
    for case in crate::prepare_test_cases("spec/markup", "spec/snapshots/lexer") {
        let symbols = test_runner::scan_str(&case.input);
        let runner = SnapTestRunner::with_fn(&case.name, &symbols, |symbols| {
            let rest = &[];
            let snapshot = Snapshot::snap((case.input.as_ref(), symbols.tokens()));
            (snapshot, rest)
        })
        .with_info(format!("Test '{}' from '{}'", case.name, case.file_name));

        unimarkup_commons::run_snap_test!(runner, &case.out_path);
    }
}
