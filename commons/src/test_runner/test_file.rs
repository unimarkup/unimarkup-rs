use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// TODO: replace with Config?
#[derive(Deserialize, Clone, Serialize)]
pub struct Preamble {
    pub title: Option<String>,
    pub author: Option<String>,
    pub lang: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct TestOutputs {
    pub html: Option<String>,
    pub um: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Test {
    pub name: String,
    pub description: Option<String>,
    pub preamble: Option<Preamble>,
    pub input: String,

    #[serde(flatten)]
    pub outputs: TestOutputs,
}

#[derive(Deserialize, Clone)]
pub struct TestFile {
    pub spec: String,
    pub name: String,
    pub description: Option<String>,
    pub preamble: Option<Preamble>,
    pub tests: Vec<Test>,
}

#[derive(Clone)]
pub struct TestCase {
    pub test: Test,
    pub file_name: String,
    pub out_path: PathBuf,
    pub file_path: String,
}
