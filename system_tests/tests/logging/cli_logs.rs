use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[test]
fn test__main_log_trace__invalid_attributes_file() {
    let path = PathBuf::from("tests/test_files/attrs.um");
    let path = path.canonicalize().unwrap();

    let cli_proc = Command::new("cargo")
        .current_dir("./..")
        .stderr(Stdio::piped())
        .args(["run", "--", "--formats=html", &path.to_string_lossy()])
        .spawn()
        .expect("Failed to spawn cargo run")
        .wait_with_output()
        .expect("Failed to execute cargo run");

    let logs = String::from_utf8_lossy(&cli_proc.stderr);

    let out_path = path.with_extension("html");

    assert!(logs.contains(&format!(
        "INFO : 536936448: Writing to file: \"{}\"",
        out_path.to_string_lossy()
    )));
    assert!(logs.contains("TRACE: 536936448: Occured in file"));
    assert!(logs.contains(&format!(
        "INFO : 536936449: Finished compiling: \"{}\"",
        path.to_string_lossy()
    )));
    assert!(logs.contains("TRACE: 536936449: Occured in file"));
}
