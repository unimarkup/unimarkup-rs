use core::fmt;
use std::{collections::VecDeque, fmt::Debug};

use pest::iterators::Pair;

use crate::{
    frontend::{
        parser::{Rule},
    }};
#[derive(Debug)]
pub struct Plain {
    pub content: String
}
#[derive(Debug)]
pub struct Bold {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Italic {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Subscript {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Superscript {
    pub content: VecDeque<FormatTypes>
}
#[derive(Debug)]
pub struct Verbatim {
    pub content: String
}
#[derive(Debug)]
pub struct Raw {
    pub content: String
}
#[derive(Debug)]
pub enum FormatTypes {
    Bold(Bold),
    Italic(Italic),
    Subscript(Subscript),
    Superscript(Superscript),
    //Verbatim(Verbatim),
    Plain(Plain),
    //TODO: Implement Raw
    //Raw(Raw)
}

pub fn create_format_types(pair: Pair<Rule>) {
    
    
    let content = get_nested_inline(pair);
    println!("{:#?}", content);
    
}

fn get_nested_inline(pair: Pair<Rule>) -> VecDeque<FormatTypes> {
    let mut content: VecDeque<FormatTypes> = VecDeque::<FormatTypes>::new();

    match pair.as_rule() {
        
        Rule::text => {
            let plain = FormatTypes::Plain(Plain { content: pair.as_str().to_string(),});
            content.push_back(plain);
        },
        Rule::italic => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let italic = Italic{content: vector};
            content.push_back(FormatTypes::Italic(italic));
        },
        Rule::subscript => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let subscript = Subscript{content: vector};
            content.push_back(FormatTypes::Subscript(subscript));
        },
        Rule::superscript => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let superscript = Superscript{content: vector};
            content.push_back(FormatTypes::Superscript(superscript));
        },
        Rule::bold => {
            let inner = pair.into_inner();
            let mut vector = VecDeque::new();

            for pair in inner {
                vector.append(&mut get_nested_inline(pair));
            }
            let bold = Bold{content: vector};
            content.push_back(FormatTypes::Bold(bold));
        },
        // TODO: No nested blocks in verbatim
        // Rule::verbatim => {
        //     let inner = pair.into_inner();
        //     let mut vector = VecDeque::new();

        //     for pair in inner {
        //         vector.append(&mut get_nested_inline(pair));
        //     }
        //     let verbatim = Verbatim{content: vector};
        //     content.push_back(FormatTypes::Verbatim(verbatim));
        // },
        _ => unreachable!("No other inline types allowed.")
    }
    
    content
} 


impl fmt::Display for FormatTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatTypes::Plain(content) => content.fmt(f),
            FormatTypes::Bold(_) => todo!(),
            FormatTypes::Italic(_) => todo!(),
            FormatTypes::Subscript(_) => todo!(),
            FormatTypes::Superscript(_) => todo!(),
            //FormatTypes::Verbatim(_) => todo!(),
            //FormatTypes::Raw(_) => todo!(),

        }
    }
}