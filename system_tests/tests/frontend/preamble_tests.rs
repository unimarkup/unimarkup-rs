use std::path::PathBuf;

use clap::StructOpt;
use pest::Parser;
use unimarkup_core::{
    config::{Config, OutputFormat},
    frontend::{
        parser::{Rule, UnimarkupParser},
        preamble::parse_preamble,
    },
};

#[test]
#[should_panic]
fn syntax_error_json() {
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
fn syntax_error_yaml() {
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
fn valid_json() {
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
    assert!(parse_preamble(pairs, &mut cfg).is_ok());
    validate_config_content(cfg);
}

#[test]
fn valid_yaml() {
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
    assert!(parse_preamble(pairs, &mut cfg).is_ok());
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
