use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::fs;

use crate::um_elements::heading_block::HeadingBlock2;

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

        if rule.as_rule().eq(&Rule::heading1)
            | rule.as_rule().eq(&Rule::heading2)
            | rule.as_rule().eq(&Rule::heading3)
            | rule.as_rule().eq(&Rule::heading4)
            | rule.as_rule().eq(&Rule::heading5)
            | rule.as_rule().eq(&Rule::heading6)
        {
            //TODO Juls: nicht wirklich elegant, noch zu ändern
            let level_heading = rule.to_string().chars().nth(7).unwrap().to_digit(10).unwrap(); //if rule headingx, get the 7th char and convert to u32

            let mut id = "".to_string();
            let mut content = "".to_string();

            for inner_rule in rule.into_inner() {
                
                if inner_rule.as_rule().eq(&Rule::text) {
                    id = inner_rule.as_str().to_string();
                } else if inner_rule.as_rule().eq(&Rule::body_heading1)
                | inner_rule.as_rule().eq(&Rule::body_heading2)
                | inner_rule.as_rule().eq(&Rule::body_heading3)
                | inner_rule.as_rule().eq(&Rule::body_heading4)
                | inner_rule.as_rule().eq(&Rule::body_heading5)
                | inner_rule.as_rule().eq(&Rule::body_heading6) {

                    content = inner_rule.as_str().to_string();
                    detect_heading(inner_rule);
                }
            }


            //detect_heading(rule);
            let parser_heading = HeadingBlock2 {
                id: id,
                level: level_heading,
                content: content,
                attributes: "".to_string(),
            };

            println!();
            println!("id: {}", parser_heading.id);
            println!("heading level: {}", parser_heading.level);
            println!("content: {}", parser_heading.content);

        } else if rule.as_rule().eq(&Rule::heading) {
            detect_heading(rule);
        }
    }
}