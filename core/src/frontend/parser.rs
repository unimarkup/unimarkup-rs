//! [`parser`](crate::frontend::parser) is the module which implements parsing of the Unimarkup syntax

use pest::{iterators::Pair, iterators::Pairs, Parser, Span};

use crate::{
    config::Config,
    elements::{
        types,
        types::{UnimarkupBlocks, UnimarkupFile},
        HeadingBlock, Metadata, MetadataKind, ParagraphBlock, VerbatimBlock,
    },
    log_id::{LogId, SetLog},
};

use super::{
    error::FrontendError,
    log_id::{ParserErrLogId, ParserWarnLogId},
    preamble,
};

/// Used to parse one specific Unimarkup block
pub trait UmParse {
    /// Parses [`UnimarkupBlocks`] from given data returned from the pest parser.
    ///
    /// # Errors
    ///
    /// This function will return an error if given Pairs of Rules contain non-valid Unimarkup syntax.
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, FrontendError>
    where
        Self: Sized;
}

/// The purpose of this module is to derive pest Parser for unimarkup grammar
mod parser_derivation {
    #![allow(missing_docs)]

    use pest_derive::Parser;

    /// UnimarkupParser which can parse a Unimarkup file into either atomic or enclosed blocks.
    ///
    /// # Rule enum
    ///
    /// The pest crate provides a proc-macro, that generates an implementation for the Parser
    /// and creates an enum Rule. Both the parser and the enum are generated according to
    /// the provided pest grammar. The Rule enum is made of variants which correspond to
    /// the individual rules in the pest grammar.
    #[allow(missing_docs)]
    #[derive(Parser)]
    #[grammar = "grammar/unimarkup.pest"]
    pub struct UnimarkupParser;
}

pub use parser_derivation::*;

/// Parses the given Unimarkup content.
///
/// Returns [`UnimarkupBlocks`] on success.
///
/// # Errors
///
/// This function will return an [`FrontendError`], if the given Unimarkup file contains invalid Unimarkup syntax.
pub fn parse_unimarkup(
    um_content: &str,
    config: &mut Config,
) -> Result<UnimarkupFile, FrontendError> {
    let mut rule_pairs = UnimarkupParser::parse(Rule::unimarkup, um_content).map_err(|err| {
        FrontendError::Parser(
            (ParserErrLogId::NoUnimarkupDetected as LogId)
                .set_log("No Unimarkup elements detected!", file!(), line!())
                .add_info(&format!("Content: '{}'", um_content))
                .add_debug(&format!("Cause: {}", err)),
        )
    })?;

    let mut unimarkup = UnimarkupFile::default();

    if let Some(um_tokens) = rule_pairs.next() {
        for pair in um_tokens.into_inner() {
            match pair.as_rule() {
                Rule::preamble => {
                    preamble::parse_preamble(pair, config)?;
                    config.validate_config()?;
                }
                Rule::atomic_block => {
                    let mut atomic_blocks = parse_atomic_block(pair)?;
                    unimarkup.blocks.append(&mut atomic_blocks);
                }
                Rule::enclosed_block => {
                    let mut enclosed_blocks = parse_enclosed_block(pair)?;

                    unimarkup.blocks.append(&mut enclosed_blocks);
                }
                Rule::blank_line | Rule::EOI => continue,
                _ => unreachable!(
                    "Unimarkup consists only of blank lines and atomic and enclosed blocks, but reached block: {:#?}",
                    pair
                ),
            }
        }
    }

    let metadata = Metadata {
        file: config.um_file.clone(),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };
    unimarkup.metadata.push(metadata);

    Ok(unimarkup)
}

fn parse_atomic_block(input: Pair<Rule>) -> Result<UnimarkupBlocks, FrontendError> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::headings, input.as_str()) {
        return HeadingBlock::parse(pairs, input.as_span());
    } else if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::paragraph, input.as_str()) {
        return ParagraphBlock::parse(pairs, input.as_span());
    }

    Ok(vec![])
}

fn parse_enclosed_block(input: Pair<Rule>) -> Result<UnimarkupBlocks, FrontendError> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::verbatim, input.as_str()) {
        return VerbatimBlock::parse(pairs, input.as_span());
    } else if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::paragraph, input.as_str()) {
        // TODO: Add implementation for the rest of enclosed blocks, return error if none of them match
        //
        // warn and fallback to paragraph for now

        (ParserWarnLogId::UnsupportedBlock as LogId)
            .set_log(
                &format!("Unsupported Unimarkup block:\n{}", input.as_str()),
                file!(),
                line!(),
            )
            .add_info("Block is parsed as a Unimarkup paragraph block.");

        return ParagraphBlock::parse(pairs, input.as_span());
    }

    Ok(vec![])
}

/// Generates a valid Unimarkup element id from non-empty string.
///
/// Unimarkup identifier has same restrictions as HTML id attribute:
///
/// 1. contains at least one character
/// 2. does not contain ASCII whitespace
///
/// The generated id preserves the case of the input string. Returns `None` if
/// input is an empty string.
///
/// # Arguments:
///
/// * `input` - non-empty input string. **Note:** input string which
/// consists only of whitespace is considered empty.
///
/// # Examples
///
/// ```rust
/// use unimarkup_core::frontend::parser::generate_id;
///
/// let input = "This is some input string";
/// assert_eq!(generate_id(input).unwrap(), "This-is-some-input-string");
/// ```
pub fn generate_id(input: &str) -> Option<String> {
    if input.trim().is_empty() {
        return None;
    }

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

    Some(result)
}

#[cfg(test)]
mod id_generator {
    #[test]
    fn test_generate_id_valid_id() {
        let input = "This is some input";
        let expect = "This-is-some-input";

        assert!(super::generate_id(input).is_some(), "generate_id is none");
        assert_eq!(
            super::generate_id(input).unwrap(),
            expect,
            "generate_id does not return expected id"
        );
    }

    #[test]
    fn test_generate_id_valid_id_with_num() {
        let input = "Th15 15 1npu7 with num6ers1";
        let expect = "Th15-15-1npu7-with-num6ers1";

        assert!(super::generate_id(input).is_some(), "generate_id is none");
        assert_eq!(
            super::generate_id(input).unwrap(),
            expect,
            "generate_id does not return expected id"
        );
    }

    #[test]
    fn test_generate_id_valid_id_many_symbols() {
        let input = "7h1$\t~1d~\t \"c0n741n$\" 'many' $ym6o1$ ~!@#$%%^&^&*()_+}{[]";
        let expect = "7h1$-~1d~-\"c0n741n$\"-'many'-$ym6o1$-~!@#$%%^&^&*()_+}{[]";

        assert!(super::generate_id(input).is_some(), "generate_id is none");
        assert_eq!(
            super::generate_id(input).unwrap(),
            expect,
            "generate_id does not return expected id"
        );
    }

    #[test]
    fn test_generate_id_empty_input() {
        let input = "";

        let id = super::generate_id(input);

        assert!(id.is_none(), "generate_id is some");
    }

    #[test]
    fn test_generate_id_whitespace_only() {
        let input = " ";

        let id = super::generate_id(input);

        assert!(id.is_none(), "generate_id is some");
    }
}
