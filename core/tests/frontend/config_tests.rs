use clap::StructOpt;
use unimarkup_core::config::Config;

#[test]
fn test__validate__valid_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    let result = cfg.validate_config();
    assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
}

#[should_panic]
#[test]
fn test__validate__invalid_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        //invalid attribute "shouldfail" on purpose
        "--dot-unimarkup=shouldfail",
        "tests/test_files/frontend/heading1.um",
    ]);

    cfg.validate_config().unwrap();
}

#[test]
fn test__validate__valid_multi_path_config() {
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

#[should_panic]
#[test]
fn test__validate__invalid_multi_path_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        //invalid attribute "shouldfail" on purpose
        "--insert-paths=shouldfail,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    cfg.validate_config().unwrap();
}

#[should_panic]
#[test]
fn test__validate__invalid_outfile_config() {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
        //invalid file "break_config_validation" on purpose
        "tests/test_files/break_config_validation.html",
    ]);

    cfg.validate_config().unwrap();
}
