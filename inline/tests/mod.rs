use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

mod lexer;
mod parser;
mod snapshot;

pub(crate) use snapshot::*;

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
