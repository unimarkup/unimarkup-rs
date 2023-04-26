use std::path::{Path, PathBuf};

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

pub fn get_test_content(test_filepath: &str) -> TestContent {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    path.push("tests/");
    let sanitized_filepath = if test_filepath.starts_with(|c| c == '/' || c == '\\') {
        &test_filepath[1..]
    } else {
        test_filepath
    };
    path.push(sanitized_filepath);

    let mut snap_path: PathBuf = path
        .components()
        .skip_while(|component| Path::new(component) != Path::new("markup"))
        .skip(1)
        .collect();

    snap_path.set_extension("");

    let input = std::fs::read_to_string(&path).unwrap();
    let test_file: crate::test_runner::test_file::TestFile = serde_yaml::from_str(&input).unwrap();

    TestContent {
        test_file,
        snap_path,
    }
}
