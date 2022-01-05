use std::collections::VecDeque;

use pest::iterators::Pair;

use crate::{
    um_error::UmError, frontend::{
        parser::{Rule},
    }};


pub enum FormatTypes {
    bold,
    italic,
    subscript,
    superscript,
    verbatim
}
pub struct InlineFormat {
    pub format: VecDeque<FormatTypes>
} 

pub fn create_format_types(pair: Pair<Rule>) {
    for rule in pair.into_inner() {
        if rule.as_rule() == Rule::formatting_type {
            
        }
    }
}

