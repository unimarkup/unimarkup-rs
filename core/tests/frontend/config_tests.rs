use clap::StructOpt;
use unimarkup_core::config::Config;

#[test]
fn test_validate_valid_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    let result = cfg.validate_config();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
}

#[test]
fn test_validate_invalid_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        //invalid attribute "shouldfail" on purpose
        "--dot-unimarkup=shouldfail",
        "tests/test_files/frontend/heading1.um",
    ]);

    assert!(
        cfg.validate_config().is_err(),
        "invalid config is detected as valid"
    );
}

#[test]
fn test_validate_valid_multi_path_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "--insert-paths=tests/test_files/,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    let result = cfg.validate_config();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
}

#[test]
fn test_validate_invalid_multi_path_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        //invalid attribute "shouldfail" on purpose
        "--insert-paths=shouldfail,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    assert!(
        cfg.validate_config().is_err(),
        "invalid config is detected as valid"
    );
}

#[test]
fn test_validate_invalid_outfile_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
        //invalid file "break_config_validation" on purpose
        "tests/test_files/break_config_validation.html",
    ]);

    assert!(
        cfg.validate_config().is_err(),
        "invalid config is detected as valid"
    );
}
