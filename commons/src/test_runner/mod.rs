use crate::scanner::{Scanner, Symbol};

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test;
pub mod test_file;

pub use insta;

/// Scans the string using the [`Scanner`] struct.
pub fn scan_str(input: &str) -> Vec<Symbol> {
    Scanner::new().scan_str(input)
}
