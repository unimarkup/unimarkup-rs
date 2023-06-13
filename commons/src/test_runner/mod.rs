use crate::scanner::{Scanner, Symbol};

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test;
pub mod test_file;

/// Scans the string with icu_testdata used as provider for Scanner.
pub fn scan_str(input: &str) -> Vec<Symbol> {
    let scanner = Scanner::try_new_with_any(icu_testdata::any()).unwrap();
    scanner.scan_str(input)
}
