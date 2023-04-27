use unimarkup_core::elements::atomic::Paragraph;

use self::snap_test_runner::SnapTestRunner;

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test;
pub mod test_file;

#[test]
pub fn paragraph_parser() {
    crate::test_parser_snap!("spec/markup/blocks/paragraph.yml", Paragraph);
}

#[test]
pub fn paragraph_with_main_parser() {
    crate::test_parser_snap!("spec/markup/blocks/paragraph.yml");
}

#[test]
pub fn paragraph_spec() {
    crate::run_spec_test!("spec/markup/blocks/paragraph.yml");
}
