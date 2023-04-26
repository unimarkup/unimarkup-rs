use std::path::{Path, PathBuf};

use unimarkup_core::elements::atomic::Paragraph;

use self::snap_test_runner::SnapTestRunner;

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test_runner;
pub mod test_file;

#[test]
pub fn paragraph_parser() {
    crate::test_parser_snap!(Paragraph, "../spec/markup/blocks/paragraph.yml");
}

#[test]
pub fn paragraph_with_main_parser() {
    crate::test_parser_snap!("../spec/markup/blocks/paragraph.yml");
}
