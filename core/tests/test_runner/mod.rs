use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use unimarkup_core::elements::atomic::Paragraph;

use crate::run_snap_test;

use self::snap_test_runner::SnapTestRunner;

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test_runner;

fn get_insta_settings() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../spec/snapshots/");

    settings
}
// TODO: replace with Config?
#[derive(Deserialize, Clone, Serialize)]
pub struct Preamble {
    pub title: Option<String>,
    pub author: Option<String>,
    pub lang: Option<String>,
}

#[derive(Deserialize)]
pub struct TestOutputs {
    pub html: Option<String>,
    pub um: Option<String>,
}

#[derive(Deserialize)]
pub struct Test {
    pub name: String,
    pub description: Option<String>,
    pub preamble: Option<Preamble>,
    pub input: String,

    #[serde(flatten)]
    pub outputs: TestOutputs,
}

#[derive(Deserialize)]
pub struct TestGroup {
    pub spec: String,
    pub name: String,
    pub description: Option<String>,
    pub preamble: Option<Preamble>,
    pub tests: Vec<Test>,
}

macro_rules! test_parser {
    ($block_ty:ty, $file_path:literal) => {
        let path = PathBuf::from(file!());
        let mut path: PathBuf = path.strip_prefix("core").unwrap().into();

        path.set_file_name("");
        path.push($file_path);

        let mut sub_path: PathBuf = path
            .components()
            .skip_while(|component| {
                dbg!(component);
                Path::new(component) != Path::new("markup")
            })
            .skip(1)
            .collect();

        sub_path.set_extension("");

        let input = std::fs::read_to_string(&path).unwrap();

        let test_group: TestGroup = serde_yaml::from_str(&input).unwrap();

        for test in &test_group.tests {
            let mut snap_runner =
                SnapTestRunner::with_parser::<$block_ty, &str>(&test.name, &test.input)
                    .with_info(file!());

            if let Some(ref description) = test.description {
                snap_runner = snap_runner.with_description(description);
            }

            let snap_runner =
                snap_runner.with_sub_path(sub_path.to_str().expect("Invalid sub path"));

            // TODO: preamble?

            $crate::run_snap_test!(snap_runner);
        }
    };
}

#[derive(Serialize)]
struct Context {
    preamble: Preamble,
}

#[test]
pub fn test_paragraph_parser() {
    test_parser!(Paragraph, "../spec/markup/blocks/paragraph.yml");
}
