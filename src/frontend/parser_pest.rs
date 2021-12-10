use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use rusqlite::Connection;
use std::fs;

use crate::{um_elements::heading_block::{HeadingBlock2, HeadingBlock}, middleend::{WriteToIr, ContentIrLine}, um_error::UmError};

extern crate pest;
extern crate pest_derive;
#[macro_use]
#[derive(Parser)]
#[grammar = "frontend/unimarkup.pest"]
pub struct UnimarkupParser;

pub fn parser_pest(ir_connection: &mut Connection) -> Result<(), UmError> {

    let mut headings_vec: Vec<HeadingBlock2> = Vec::new();
    let ir_transaction = ir_connection.transaction();

    let src = fs::read_to_string("src/frontend/textfiles/test.txt").expect("cannot read file");
    let parsed_src = UnimarkupParser::parse(Rule::file, &src)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    detect_heading(parsed_src, & mut headings_vec);


    if let Ok(transaction) = ir_transaction {
        for element in headings_vec {
            let ir_lines : Vec<ContentIrLine> = element.into();
            for ir_line in ir_lines {
                ir_line.write_to_ir(&transaction)?;
            }
        }
        transaction.commit().unwrap();
    }

    Ok(())
}

pub fn detect_heading(parsed_src: Pair<Rule>, headings_vec: & mut Vec<HeadingBlock2>) {

    for rule in parsed_src.into_inner() {
        // println!("{}", rule);
        // println!("------------");

        if rule.as_rule().eq(&Rule::heading1)
            | rule.as_rule().eq(&Rule::heading2)
            | rule.as_rule().eq(&Rule::heading3)
            | rule.as_rule().eq(&Rule::heading4)
            | rule.as_rule().eq(&Rule::heading5)
            | rule.as_rule().eq(&Rule::heading6)
        {
            //TODO Juls: nicht wirklich elegant, noch zu ändern
            let level_heading = rule.as_rule().into();
            let mut id = "".to_string();
            let mut content = "".to_string();
            let (line_number,_) = rule.as_span().start_pos().line_col();

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
                    //detect_heading(inner_rule, headings_vec);
                }
            }
            
            //detect_heading(rule);
            let parser_heading = HeadingBlock2 {
                id,
                level: level_heading,
                content,
                attributes: "".to_string(),
                line_number
            };

            // println!();
            // println!("id: {}", parser_heading.id);
            // println!("heading level: {}", parser_heading.level);
            // println!("content: {}", parser_heading.content);

            headings_vec.push(parser_heading);
            
        } else if rule.as_rule().eq(&Rule::heading) {
            detect_heading(rule, headings_vec);
        }
    }

    //headings_vec
}