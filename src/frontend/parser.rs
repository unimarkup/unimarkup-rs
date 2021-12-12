use pest::{iterators::Pair, iterators::Pairs, Parser, Span};
use pest_derive::Parser;
use std::{fs, path::Path};

use crate::{um_elements::heading_block::HeadingBlock, um_error::UmError};

use super::UnimarkupBlocks;

pub trait UmParse {
    fn parse_multiple(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, UmError>
    where
        Self: Sized;
    fn parse(pair: Pair<Rule>) -> Self
    where
        Self: Sized;
}

#[derive(Parser)]
#[grammar = "grammar/unimarkup.pest"]
pub struct UnimarkupParser;

pub fn parse_unimarkup(um_file: &Path) -> Result<UnimarkupBlocks, UmError> {
    let source = fs::read_to_string(um_file).map_err(|err| UmError::General {
        msg: String::from("Could not read file."),
        error: Box::new(err),
    })?;

    let mut rule_pairs =
        UnimarkupParser::parse(Rule::unimarkup, &source).map_err(|err| UmError::General {
            msg: String::from("Could not parse file!"),
            error: Box::new(err),
        })?;

    let mut blocks: UnimarkupBlocks = Vec::new();

    if let Some(unimarkup) = rule_pairs.next() {
        for pair in unimarkup.into_inner() {
            if pair.as_rule() == Rule::atomic_block {
                let mut atomic_blocks = parse_atomic_block(&pair)?;
                blocks.append(&mut atomic_blocks);
            }
        }
    }

    Ok(blocks)
}

fn parse_atomic_block(input: &Pair<Rule>) -> Result<UnimarkupBlocks, UmError> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::headings, input.as_str()) {
        return HeadingBlock::parse_multiple(pairs, input.as_span());
    }

    Ok(vec![])
}
