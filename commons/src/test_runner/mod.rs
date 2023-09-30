use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use crate::scanner::{Scanner, Symbol};

pub mod as_snapshot;
pub mod snap_test_runner;
pub mod spec_test;
pub mod test_file;

use self::test_file::{TestCase, TestFile};

/// Scans the string using the [`Scanner`] struct.
pub fn scan_str(input: &str) -> Vec<Symbol> {
    let scanner = Scanner::default();
    scanner.scan_str(input)
}

/// Finds all files with the given extension in the given path (recursively).
///
/// # Arguments
///
/// * `path` - The path to the directory to search in.
/// * `ext` - The extension of the files that should be included
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
/// * `out_folder` - The folder to write the snapshots to.
/// * `test_file` - The path to the input test file. Path will be generated with the help of
///   `test_file`'s name.
/// * `separator` - component of the `test_file` [`Path`], after which all the components of that
///    path will be included in the generated output path.
///
/// # Example
/// ```rust
/// use unimarkup_commons::test_runner::gen_snap_path;
/// use std::path::Path;
///
/// let out_folder      = "tests/spec/snapshots/lexer";
/// let input_file_path = "tests/spec/markup/some-subfolder/underline.yml";
///                                       // ^^^^^^^^^^^^^^^^^^^^^^^^ will be contained in generated output path
/// let path = gen_snap_path(out_folder, input_file_path, "markup");
///
/// assert_eq!(&path, Path::new("tests/spec/snapshots/lexer/some-subfolder/underline"));
/// ```
pub fn gen_snap_path(
    out_folder: impl AsRef<Path>,
    test_file: impl AsRef<Path>,
    separator: &str,
) -> PathBuf {
    let mut path = PathBuf::from(out_folder.as_ref());

    let mut snap_path: PathBuf = test_file
        .as_ref()
        .components()
        .skip_while(|component| Path::new(component) != Path::new(separator))
        .skip(1)
        .collect();

    snap_path.set_extension("");

    path.push(snap_path);
    path
}

/// Generates test cases from test files
///
/// # Arguments
/// * `markups` - Path to the folder containing the (input) test files. Test files are in YML
///    format (have extension '.yml') and will be collected recursively.
/// * `output` - Path to the folder where the (output) snapshot files will be saved.
/// * `separator` - The component of the `markups` Path, such that all the components of the path
///    following it will be included in the generated output path. These components will be
///    appended to the `output` path.
pub fn collect_tests(
    markups: impl AsRef<Path>,
    output: impl AsRef<Path>,
    separator: &str,
) -> Vec<TestCase> {
    let markups_path = PathBuf::from(markups.as_ref());

    let entries = collect_entries(markups_path, "yml").unwrap();

    let cases = entries.iter().flat_map(|entry| {
        let path = entry.path();
        let input = std::fs::read_to_string(&path).unwrap();

        let mut test_file: TestFile = serde_yaml::from_str(&input).unwrap();

        let output = output.as_ref();
        let cases = test_file.tests.drain(..).map(move |test| {
            let file_name = path.file_name().and_then(|file| file.to_str()).unwrap();
            let out_path = gen_snap_path(output, &path, separator);

            let file_path: String = path
                .components()
                .skip_while(|c| Path::new(c) != Path::new(separator))
                .collect::<PathBuf>()
                .to_string_lossy()
                .to_string();

            TestCase {
                test,
                file_name: String::from(file_name),
                out_path,
                file_path,
            }
        });

        cases.collect::<Vec<_>>()
    });

    cases.collect()
}

/// Returns the absolute path to the integration `tests` folder of the crate it's called from.
#[macro_export]
macro_rules! crate_tests_path {
    () => {{
        let curr = ::std::env!("CARGO_MANIFEST_DIR");

        let mut buf = std::path::PathBuf::from(curr);
        buf.push("tests");

        buf
    }};
}
