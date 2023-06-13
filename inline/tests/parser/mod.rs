use crate::snapshot::Snapshot;
use unimarkup_commons::test_runner::{self, snap_test_runner::SnapTestRunner};
use unimarkup_inline::ParseInlines;

mod snapshot;

#[test]
fn test_parser_snapshots() {
    for case in crate::prepare_test_cases("spec/markup", "spec/snapshots/parser") {
        let symbols = test_runner::scan_str(&case.input);

        let runner = SnapTestRunner::with_fn(&case.name, &symbols, |symbols| {
            let rest: &[_] = &[];
            let inlines: Vec<_> = symbols.parse_inlines().collect();
            let snapshot = Snapshot::snap(&inlines[..]);
            (snapshot, rest)
        })
        .with_info(format!("Test '{}' from '{}'", case.name, case.file_name));

        unimarkup_commons::run_snap_test!(runner, &case.out_path);
    }
}
