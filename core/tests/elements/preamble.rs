use std::path::PathBuf;

use clap::StructOpt;
use pest::Parser;
use unimarkup_core::{
    config::{Config, OutputFormat},
    elements::preamble::parse_preamble,
    frontend::parser::{Rule, UnimarkupParser},
};

#[test]
#[should_panic]
fn test__parse__invalid_preamble_json() {
    //Invalid missing quotation mark at OUTPUT-FILE on purpose
    let test_case = ";;;
{
    OUTPUT-FILE\": \"output.html\",
    \"citation-style\": \"yes\",
    \"output-formats\": [\"Html\"],
    \"html_embed_svg\": true
}
;;;
    
    ";

    let mut cfg: Config = create_test_config();
    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();

    parse_preamble(pairs, &mut cfg).unwrap();
}

#[test]
#[should_panic]
fn test__parse__invalid_preamble_yaml() {
    //Invalid extra commas on purpose
    let test_case = ";;;
    OUTPUT-FILE: \"output.html\",
    citation-style: \"yes\",
    output-formats: [\"Html\"],
    html_embed_svg: true
;;;
    
    ";

    let mut cfg: Config = create_test_config();
    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();

    parse_preamble(pairs, &mut cfg).unwrap();
}

#[test]
fn test__parse__valid_preamble_json() {
    let test_case = ";;;
{
    \"OUTPUT-FILE\": \"output.html\",
    \"citation-style\": \"yes\",
    \"output-formats\": [\"Html\"],
    \"html_embed_svg\": true
}
;;;
    
    ";

    let mut cfg: Config = create_test_config();
    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();
    assert!(
        parse_preamble(pairs, &mut cfg).is_ok(),
        "Valid preamble in form of json is not detected as valid"
    );
    validate_config_content(cfg);
}

#[test]
fn test__parse__valid_preamble_yaml() {
    let test_case = ";;;
OUTPUT-FILE: \"output.html\"
citation-style: \"yes\"
output-formats: [\"Html\"]
html_embed_svg: true
;;;
    
    ";

    let mut cfg: Config = create_test_config();
    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();
    assert!(
        parse_preamble(pairs, &mut cfg).is_ok(),
        "Valid preamble in form of yaml is not detected as valid"
    );
    validate_config_content(cfg);
}

fn validate_config_content(config: Config) {
    let out_formats: Vec<OutputFormat> = vec![OutputFormat::Html];
    let out_file = PathBuf::from("output.html");
    let citation_style = PathBuf::from("yes");

    assert_eq!(config.out_file.unwrap(), out_file);
    assert_eq!(config.citation_style.unwrap(), citation_style);
    assert_eq!(config.out_formats.unwrap(), out_formats);
    assert!(config.html_embed_svg);
}

fn create_test_config() -> Config {
    let cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
    ]);
    cfg
}
