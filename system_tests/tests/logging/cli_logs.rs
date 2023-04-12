use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

const TEST_FILE: &str = "attrs.um";

#[test]
fn test__main_log_trace__attributes_file() {
    let path = get_path();

    let cli_proc = Command::new("cargo")
        .current_dir(get_proc_path())
        .stdout(Stdio::piped())
        .args(["run", "--", "--formats=html", &path.to_string_lossy()])
        .spawn()
        .expect("Failed to spawn 'cargo run'");

    let output = cli_proc
        .wait_with_output()
        .expect("Failed to execute 'cargo run'");
    let logs = String::from_utf8_lossy(&output.stdout);

    assert!(logs.contains("64: Writing to file: "));
    assert!(logs.contains(&TEST_FILE.replace(".um", ".html")));
    assert!(logs.contains("64(origin): file="));
    assert!(logs.contains("65: Finished compiling: "));
    assert!(logs.contains(&TEST_FILE));
    assert!(logs.contains("65(origin): file="));
}

// Note: Functions below needed to get the test running in 'run' and 'debug' mode

fn get_path() -> PathBuf {
    let filePath = PathBuf::from(file!());
    let fileRoot = filePath.parent().unwrap();
    let path = fileRoot.join("../test_files/".to_owned() + TEST_FILE);
    match path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            let path = PathBuf::from("tests/test_files/".to_owned() + TEST_FILE);
            path.canonicalize().unwrap()
        }
    }
}

fn get_proc_path() -> PathBuf {
    let filePath = PathBuf::from(file!());
    let fileRoot = filePath.parent().unwrap();
    let repoPath = fileRoot.join("../../../.");
    if fileRoot.canonicalize().is_ok() {
        if let Ok(path) = repoPath.canonicalize() {
            return path;
        }
    }

    PathBuf::from("./..")
}
