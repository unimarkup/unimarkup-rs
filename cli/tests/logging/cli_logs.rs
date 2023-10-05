use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

const TEST_FILE: &str = "supported.um";

#[test]
fn log_output_set_correctly() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    path.push(format!("tests/test_files/{}", TEST_FILE));

    let cli_proc = Command::new("cargo")
        .stderr(Stdio::piped())
        .args([
            "run",
            "--",
            "--formats=html",
            "--lang=en",
            &path.to_string_lossy(),
        ])
        .spawn()
        .expect("Failed to spawn 'cargo run'");

    let output = cli_proc
        .wait_with_output()
        .expect("Failed to execute 'cargo run'");
    let logs = String::from_utf8_lossy(&output.stderr);

    assert!(logs.contains("Writing to file: "));
    assert!(logs.contains(&TEST_FILE.replace(".um", ".html")));
    assert!(logs.contains("Unimarkup finished compiling."));
}
