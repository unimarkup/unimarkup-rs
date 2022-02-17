use std::process::{Command, Stdio};

#[test]
fn test_main_log_trace() {
    let cli_proc = Command::new("cargo")
        .stderr(Stdio::piped())
        .args([
            "run",
            "--bin",
            "cli",
            "--",
            "--formats=html",
            "tests/test_files/attrs.um",
        ])
        .spawn()
        .expect("Failed to spawn cargo run")
        .wait_with_output()
        .expect("Failed to execute cargo run");

    let logs = String::from_utf8_lossy(&cli_proc.stderr);

    assert!(logs.contains("INFO : 536936448: Writing to file: \"tests/test_files/attrs.html\""));
    assert!(logs.contains("TRACE: 536936448: Occured in file"));
    assert!(logs.contains("INFO : 536936449: Finished compiling: \"tests/test_files/attrs.um\""));
    assert!(logs.contains("TRACE: 536936449: Occured in file"));
}
