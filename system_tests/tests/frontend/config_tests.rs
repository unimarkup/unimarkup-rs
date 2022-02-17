use clap::StructOpt;
use unimarkup_core::config::Config;

#[test]
fn test_valid_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    cfg.validate_config().unwrap();
}

#[test]
fn test_invalid_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=shouldfail",
        "tests/test_files/frontend/heading1.um",
    ]);

    assert!(cfg.validate_config().is_err());
}

#[test]
fn test_valid_multi_path_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "--insert-paths=tests/test_files/,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    cfg.validate_config().unwrap();
}

#[test]
fn test_invalid_multi_path_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "--insert-paths=shouldfail,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    assert!(cfg.validate_config().is_err());
}
