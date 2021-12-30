//! [`parser`](crate::frontend::parser) is the module which implements parsing of the Unimarkup syntax

use pest::{iterators::Pair, iterators::Pairs, Parser, Span};
use pest_derive::Parser;
use std::{fs, path::Path};

use crate::um_elements::types;
use crate::{um_elements::HeadingBlock, um_elements::ParagraphBlock, um_error::UmError};

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
            if pair.as_rule() == Rule::atomic_block {
                let mut atomic_blocks = parse_atomic_block(pair)?;
                blocks.append(&mut atomic_blocks);
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

/// Generates a valid Unimarkup element id from non-empty string.
///
/// # Arguments:
/// * `input` - non-empty input string. **Note:** input string which
/// consists only of whitespace is considered empty.
///
/// # Examples
///
/// ```rust
/// use unimarkup_rs::frontend::parser::generate_id;
///
/// let input = "This is some input string";
/// assert_eq!(generate_id(input), "This-is-some-input-string");
/// ```
///
/// ## Empty (or whitespace-only) inputs cause panic
/// ```rust,should_panic
/// use unimarkup_rs::frontend::parser::generate_id;
///
/// // This panics
/// let id = generate_id(" ");
/// ```
///
/// # Panics
///
/// Panics if input is either empty or contains only whitespace.
pub fn generate_id(input: &str) -> String {
    // Unimarkup identifier has same restrictions as HTML id attribute:
    // 1. contains at least one character
    // 2. does not contain ASCII whitespace

    assert!(
        !input.trim().is_empty(),
        "Input string is either empty or consists only of whitespace(s)!"
    );

    let result = {
        let mut id = String::new();

        for (i, word) in input.split_whitespace().enumerate() {
            if i != 0 {
                id.push(types::DELIMITER);
            }

            id.push_str(word);
        }

        id
    };

    result
}

#[cfg(test)]
mod id_generator {
    #[test]
    fn valid_id() {
        let input = "This is some input";
        let expect = "This-is-some-input";

        assert_eq!(super::generate_id(input), expect);
    }

    #[test]
    fn valid_id_with_num() {
        let input = "Th15 15 1npu7 with num6ers1";
        let expect = "Th15-15-1npu7-with-num6ers1";

        assert_eq!(super::generate_id(input), expect);
    }

    #[test]
    fn valid_id_many_symbols() {
        let input = "7h1$\t~1d~\t \"c0n741n$\" 'many' $ym6o1$ ~!@#$%%^&^&*()_+}{[]";
        let expect = "7h1$-~1d~-\"c0n741n$\"-'many'-$ym6o1$-~!@#$%%^&^&*()_+}{[]";

        assert_eq!(super::generate_id(input), expect);
    }

    #[test]
    #[should_panic]
    fn empty_input() {
        let input = "";

        let id = super::generate_id(input);

        assert!(!id.is_empty());
    }

    #[test]
    #[should_panic]
    fn whitespace_only() {
        let input = " ";

        let id = super::generate_id(input);

        assert!(!id.is_empty());
    }
}
