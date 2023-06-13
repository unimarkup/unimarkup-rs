use crate::{
    config::Config,
    scanner::{Scanner, Symbol},
};

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test;
pub mod test_file;

/// Scans the string using Scanner with icu_provider constructed from default
/// icu locale data.
pub fn scan_str(input: &str) -> Vec<Symbol> {
    let cfg = Config::default();

    let scanner = Scanner::try_new(cfg.icu_provider()).unwrap();
    scanner.scan_str(input)
}
