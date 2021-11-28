use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::fs;

extern crate pest;
extern crate pest_derive;
#[macro_use]
#[derive(Parser)]
#[grammar = "frontend/unimarkup.pest"]
pub struct UnimarkupParser;

pub fn parser_pest() {
    let src = fs::read_to_string("src/frontend/textfiles/test.txt").expect("cannot read file");
    let parsed_src = UnimarkupParser::parse(Rule::file, &src)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    detect_heading(parsed_src);
}

pub fn detect_heading(parsed_src: Pair<Rule>) {
    for rule in parsed_src.into_inner() {
        println!("{}", rule);
        println!("------------");

        if rule.as_rule().eq(&Rule::body_heading1)
            | rule.as_rule().eq(&Rule::body_heading2)
            | rule.as_rule().eq(&Rule::body_heading3)
            | rule.as_rule().eq(&Rule::body_heading4)
            | rule.as_rule().eq(&Rule::body_heading5)
            | rule.as_rule().eq(&Rule::body_heading6)
            | rule.as_rule().eq(&Rule::heading1)
            | rule.as_rule().eq(&Rule::heading2)
            | rule.as_rule().eq(&Rule::heading3)
            | rule.as_rule().eq(&Rule::heading4)
            | rule.as_rule().eq(&Rule::heading5)
            | rule.as_rule().eq(&Rule::heading6)
            | rule.as_rule().eq(&Rule::heading)
        {
            detect_heading(rule);
        }
    }
}

pub struct Heading {
    rule: Rule,
    body: String,
}
