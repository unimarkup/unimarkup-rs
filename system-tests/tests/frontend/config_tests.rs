use clap::StructOpt;
use unimarkup_core::{config::Config, error::UmError};

#[test]
fn test_valid_config() -> Result<(), UmError> {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    cfg.validate_config()?;

    Ok(())
}

#[test]
fn test_invalid_config() -> Result<(), UmError> {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=shouldfail",
        "tests/test_files/frontend/heading1.um",
    ]);

    assert!(cfg.validate_config().is_err());

    Ok(())
}

#[test]
fn test_valid_multi_path_config() -> Result<(), UmError> {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "--insert-paths=tests/test_files/,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    cfg.validate_config()?;

    Ok(())
}

#[test]
fn test_invalid_multi_path_config() -> Result<(), UmError> {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "--dot-unimarkup=tests/test_files/",
        "--insert-paths=shouldfail,tests/test_files/",
        "tests/test_files/frontend/heading1.um",
    ]);

    assert!(cfg.validate_config().is_err());

    Ok(())
}

#[test]
fn test_invalid_outfile_config() -> Result<(), UmError> {
    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
        "tests/test_files/break_config_validation.html",
    ]);

    assert!(cfg.validate_config().is_err());

    Ok(())
}
