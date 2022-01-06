//! [`parser`](crate::frontend::parser) is the module which implements parsing of the Unimarkup syntax

use pest::{iterators::Pair, iterators::Pairs, Parser, Span};
use pest_derive::Parser;
use std::{fs, path::Path};

use crate::{
    um_elements::HeadingBlock,
    um_elements::{ParagraphBlock, VerbatimBlock},
    um_error::UmError,
};

use super::UnimarkupBlocks;

/// Used to parse one specific Unimarkup block
pub trait UmParse {
    /// Parses [`UnimarkupBlocks`] from given data returned from the pest parser.
    ///
    /// # Errors
    ///
    /// This function will return an error if given Pairs of Rules contain non-valid Unimarkup syntax.
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, UmError>
    where
        Self: Sized;
}

/// The purpose of this module is to derive pest Parser for unimarkup grammar
mod parser_derivation {
    #![allow(missing_docs)]

    use super::*;

    /// UnimarkupParser which can parse a Unimarkup file into either atomic or enclosed blocks.
    ///
    /// # Rule enum
    ///
    /// The pest crate provides a proc-macro, that generates an implementation for the Parser
    /// and creates an enum Rule. Both the parser and the enum are generated according to
    /// the provided pest grammar. The Rule enum is made of variants which correspond to
    /// the individual rules in the pest grammar.
    #[derive(Parser)]
    #[allow(missing_docs)]
    #[grammar = "grammar/unimarkup.pest"]
    pub struct UnimarkupParser;
}

pub use parser_derivation::*;

/// Parses the given Unimarkup file.
///
/// Returns [`UnimarkupBlocks`] on success.
///
/// # Errors
///
/// This function will return an [`UmError`], if the given Unimarkup file contains invalid Unimarkup syntax.
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
            match pair.as_rule() {
                Rule::atomic_block => {
                    let mut atomic_blocks = parse_atomic_block(pair)?;
                    blocks.append(&mut atomic_blocks);
                }
                Rule::enclosed_block => {
                    let mut enclosed_blocks = parse_enclosed_block(pair)?;

                    blocks.append(&mut enclosed_blocks);
                }
                Rule::blank_line | Rule::EOI => continue,
                _ => unreachable!(
                    "Unimarkup consists only of blank lines, atomic and enclosed blocks, but reached block: {:#?}",
                    pair
                ),
            }
        }
    }

    Ok(blocks)
}

fn parse_atomic_block(input: Pair<Rule>) -> Result<UnimarkupBlocks, UmError> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::headings, input.as_str()) {
        return HeadingBlock::parse(pairs, input.as_span());
    } else if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::paragraph, input.as_str()) {
        return ParagraphBlock::parse(pairs, input.as_span());
    }

    Ok(vec![])
}

fn parse_enclosed_block(input: Pair<Rule>) -> Result<UnimarkupBlocks, UmError> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::verbatim, input.as_str()) {
        return VerbatimBlock::parse(pairs, input.as_span());
    } else if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::paragraph, input.as_str()) {
        // fallback to paragraph for now
        return ParagraphBlock::parse(pairs, input.as_span());
    }

    Ok(vec![])
}
