use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

mod lexer;
mod parser;
mod snapshot;

use lexer::test_lexer_snapshots;
use libtest_mimic::Arguments;
use parser::test_parser_snapshots;
pub(crate) use snapshot::*;
use unimarkup_commons::test_runner::test_file::TestFile;

fn main() {
    let args = Arguments::from_args();
    let lexer_tests = test_lexer_snapshots();
    let mut parser_tests = test_parser_snapshots();

    let mut tests = lexer_tests;
    tests.append(&mut parser_tests);

    libtest_mimic::run(&args, tests).exit();
}

pub struct TestCase {
    name: String,
    input: String,
    file_name: String,
    out_path: PathBuf,
}

/// Generates test cases from test files
///
/// # Arguments
/// * `markups` - Path to the folder containing the (input) test files
/// * `output` - Path to the folder where the (output) snapshot files will be saved
pub fn prepare_test_cases(markups: impl AsRef<Path>, output: impl AsRef<Path>) -> Vec<TestCase> {
    let mut markups_path = crate::tests_path();
    markups_path.push(markups);

    let entries = crate::collect_entries(markups_path, "yml").unwrap();

    let cases = entries.iter().flat_map(|entry| {
        let path = entry.path();
        let input = std::fs::read_to_string(&path).unwrap();

        let mut test_file: TestFile = serde_yaml::from_str(&input).unwrap();

        let output = output.as_ref();
        let cases = test_file.tests.drain(..).map(move |test| {
            let file_name = path.file_name().and_then(|file| file.to_str()).unwrap();
            let out_path = crate::gen_snap_path(output, &path);

            TestCase {
                name: test.name,
                input: test.input,
                file_name: String::from(file_name),
                out_path,
            }
        });

        cases.collect::<Vec<_>>()
    });

    cases.collect()
}

/// Returns the absolute path to the integration `tests` folder.
pub fn tests_path() -> PathBuf {
    let curr = std::env!("CARGO_MANIFEST_DIR");

    let mut buf = PathBuf::from(curr);
    buf.push("tests");

    buf
}

/// Finds all files with the given extension in the given path (recursively).
pub fn collect_entries(path: impl AsRef<Path>, ext: &str) -> std::io::Result<Vec<DirEntry>> {
    let mut entries = vec![];

    if path.as_ref().is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                entries.append(&mut collect_entries(path, ext)?);
            } else if path.extension().and_then(|ext| ext.to_str()).unwrap_or("") == ext {
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

/// Generates the output path for the given test name.
///
/// # Arguments
///
/// * `out_folder` - The folder to write the snapshots to, relative to the `tests` folder.
/// * `test_file` - The path to the input test file relative to the integration `tests`
///    folder. Path will be generated with the help of `test_file`'s name.
///
/// # Example
/// ```
/// use unimarkup_inline::gen_snap_path;
///
/// let in_file_path = "spec/markup/underline.yml";
/// let path = gen_snap_path("spec/snapshots/lexer");
///
/// assert_eq!(&path, Path::new("tests/spec/snapshots/lexer/markup/underline"));
/// ```
pub fn gen_snap_path<F, T>(out_folder: F, test_file: T) -> PathBuf
where
    F: AsRef<Path>,
    T: AsRef<Path>,
{
    let mut path = crate::tests_path();
    path.push(out_folder);

    let mut snap_path: PathBuf = test_file
        .as_ref()
        .components()
        .skip_while(|component| Path::new(component) != Path::new("markup"))
        .skip(1)
        .collect();

    snap_path.set_extension("");

    path.push(snap_path);
    path
}

#[test]
fn test_gen_snap_path() {
    let in_file_path = Path::new("spec/tmp/markup/temp_file.yml");

    let path = gen_snap_path("spec/tmp/snapshot/", in_file_path);

    let mut expected = crate::tests_path();
    expected.push("spec/tmp/snapshot/temp_file");

    assert_eq!(path, expected.as_path());
}
