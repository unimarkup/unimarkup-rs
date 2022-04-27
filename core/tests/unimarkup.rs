#![allow(non_snake_case)]

use clap::Parser;
use unimarkup_core::config::Config;

#[test]
fn test__compile__empty_content() {
    let cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/all_syntax.um",
    ]);

    let rendered_result = unimarkup_core::unimarkup::compile("", cfg);

    assert!(rendered_result.unwrap().elements.is_empty());
}
