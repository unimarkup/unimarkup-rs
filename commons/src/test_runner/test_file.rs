use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
pub struct TestFile {
    pub spec: String,
    pub name: String,
    pub description: Option<String>,
    pub preamble: Option<Preamble>,
    pub tests: Vec<Test>,
}

pub struct TestContent {
    pub test_file: TestFile,
    pub snap_path: PathBuf,
}

pub struct TestCase {
    pub test: Test,
    pub file_name: String,
    pub out_path: PathBuf,
}

pub fn get_test_content(test_filepath: PathBuf, snap_path: PathBuf) -> TestContent {
    let input = std::fs::read_to_string(test_filepath).unwrap();
    let test_file: crate::test_runner::test_file::TestFile = serde_yaml::from_str(&input).unwrap();

    TestContent {
        test_file,
        snap_path,
    }
}
