use std::ops::Deref;
use std::path::{Path, PathBuf};

use unimarkup_commons::run_spec_test;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_commons::test_runner::snap_test_runner::SnapTestRunner;
use unimarkup_commons::test_runner::test_file::{Test, TestOutputs};
use unimarkup_parser::elements::atomic::Paragraph;
use unimarkup_parser::elements::blocks::Block;
use unimarkup_parser::elements::Blocks;
use unimarkup_parser::ParserGenerator;

#[derive(Debug)]
pub(crate) struct Snapshot<T>(T);

impl<T> Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsSnapshot for Snapshot<Blocks> {
    fn as_snapshot(&self) -> String {
        self.0
            .iter()
            .map(|block| Snapshot(block).as_snapshot())
            .collect()
    }
}

impl AsSnapshot for Snapshot<&Block> {
    fn as_snapshot(&self) -> String {
        match **self {
            Block::Paragraph(block) => Snapshot(block).as_snapshot(),
            _ => unimplemented!("TODO: Implement snapshot for {:?}", self),
        }
    }
}

macro_rules! snapshot_parser {
    ($ty:ty) => {
        |input| {
            let parse = <$ty>::generate_parser();

            parse(input.into())
                .map(|(block, rest)| (Snapshot(block).as_snapshot(), rest))
                .expect("Could not parse content!")
        }
    };
}

macro_rules! spec_parser {
    () => {
        |test: &Test, cfg| {
            let input = test.input.trim_end();

            let um = unimarkup_core::Unimarkup::parse(input, cfg);

            TestOutputs {
                html: Some(um.render_html().unwrap().to_string()),
                um: Some(test.input.clone()),
            }
        }
    };
}

pub fn gen_path(input_path: &str) -> (PathBuf, PathBuf) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();

    path.push("tests/");
    let sanitized_filepath = if input_path.starts_with(|c| c == '/' || c == '\\') {
        &input_path[1..]
    } else {
        input_path
    };
    path.push(sanitized_filepath);

    let mut snap_path: PathBuf = path
        .components()
        .skip_while(|component| Path::new(component) != Path::new("markup"))
        .skip(1)
        .collect();

    snap_path.set_extension("");

    (path, snap_path)
}

#[test]
pub fn paragraph_parser() {
    unimarkup_commons::test_parser_snap!(
        gen_path("spec/markup/blocks/paragraph.yml"),
        snapshot_parser!(Paragraph)
    );
}

#[test]
pub fn paragraph_with_main_parser() {
    unimarkup_commons::test_parser_snap!(
        gen_path("spec/markup/blocks/paragraph.yml"),
        snapshot_parser!(Paragraph)
    );
}

#[test]
pub fn paragraph_spec() {
    run_spec_test!(gen_path("spec/markup/blocks/paragraph.yml"), spec_parser!());
}
