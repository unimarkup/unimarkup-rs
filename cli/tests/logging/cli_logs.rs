// use std::{
//     path::PathBuf,
//     process::{Command, Stdio},
// };
//
// const TEST_FILE: &str = "attrs.um";

// TODO: rework log tests after updating logid
// #[test]
// fn test__main_log_trace__attributes_file() {
//     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//         .canonicalize()
//         .unwrap();
//     path.push("tests/test_files/attrs.um");
//
//     let cli_proc = Command::new("cargo")
//         .stdout(Stdio::piped())
//         .args(["run", "--", "--formats=html", &path.to_string_lossy()])
//         .spawn()
//         .expect("Failed to spawn 'cargo run'");
//
//     let output = cli_proc
//         .wait_with_output()
//         .expect("Failed to execute 'cargo run'");
//     let logs = String::from_utf8_lossy(&output.stdout);
//
//     assert!(logs.contains("64: Writing to file: "));
//     assert!(logs.contains(&TEST_FILE.replace(".um", ".html")));
//     assert!(logs.contains("64(origin): file="));
//     assert!(logs.contains("65: Finished compiling: "));
//     assert!(logs.contains(TEST_FILE));
//     assert!(logs.contains("65(origin): file="));
// }
