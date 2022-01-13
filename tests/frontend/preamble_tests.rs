use clap::StructOpt;
use pest::Parser;
use unimarkup_rs::{
    config::Config,
    frontend::{
        parser::{Rule, UnimarkupParser},
        preamble::parse_preamble,
    },
};

#[test]
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

    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
    ]);

    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();
    let err = parse_preamble(pairs, &mut cfg).unwrap_err();
    assert!(err.to_string().contains("Expected JSON"));
}

#[test]
fn syntax_error_yaml() {
    let test_case = ";;;
    OUTPUT-FILE: \"output.html\",
    citation-style: \"yes\",
    output-formats: [\"Html\"],
    html_embed_svg: true
;;;
    
    ";

    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
    ]);

    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();
    let err = parse_preamble(pairs, &mut cfg).unwrap_err();
    assert!(err.to_string().contains("Expected YAML"));
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

    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
    ]);

    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();
    assert!(parse_preamble(pairs, &mut cfg).is_ok());
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

    let mut cfg: Config = Config::parse_from(vec![
        "unimarkup",
        "--output-formats=html",
        "tests/test_files/frontend/heading1.um",
    ]);

    let pairs = UnimarkupParser::parse(Rule::preamble, test_case)
        .expect("test")
        .next()
        .unwrap();
    assert!(parse_preamble(pairs, &mut cfg).is_ok());
}
