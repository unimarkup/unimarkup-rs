//! [`parser`](crate::frontend::parser) is the module which implements parsing of the Unimarkup syntax

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use pest::{iterators::Pair, iterators::Pairs, Parser, Span};

use crate::{
    config::Config,
    document::Document,
    elements::{
        types, HeadingBlock, Metadata, MetadataKind, ParagraphBlock, UnimarkupBlocks, VerbatimBlock,
    },
    log_id::CORE_LOG_ID_MAP,
    security,
};

use super::{
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
    fn parse(pairs: &mut Pairs<Rule>, span: Span) -> Result<UnimarkupBlocks, MappedLogId>
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
pub fn parse_unimarkup(um_content: &str, config: &mut Config) -> Result<Document, MappedLogId> {
    let mut rule_pairs = UnimarkupParser::parse(Rule::unimarkup, um_content).map_err(|err| {
        (ParserErrLogId::NoUnimarkupDetected as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
                "No Unimarkup elements detected!",
                file!(),
                line!(),
            )
            .add_info(&format!("Content: '{}'", um_content))
            .add_cause(&format!("{}", err))
    })?;

    let mut unimarkup = Document::default();

    if let Some(um_tokens) = rule_pairs.next() {
        for pair in um_tokens.into_inner() {
            match pair.as_rule() {
                Rule::preamble => {
                    preamble::parse_preamble(pair, config)?;
                    config.validate_config()?;
                }
                Rule::atomic_block => {
                    let mut atomic_blocks = parse_atomic_block(pair)?;
                    unimarkup.elements.append(&mut atomic_blocks);
                }
                Rule::enclosed_block => {
                    let mut enclosed_blocks = parse_enclosed_block(pair)?;

                    unimarkup.elements.append(&mut enclosed_blocks);
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
        contenthash: security::get_contenthash(um_content),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };
    unimarkup.metadata.push(metadata);

    Ok(unimarkup)
}

fn parse_atomic_block(input: Pair<Rule>) -> Result<UnimarkupBlocks, MappedLogId> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::headings, input.as_str()) {
        return HeadingBlock::parse(pairs, input.as_span());
    } else if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::paragraph, input.as_str()) {
        return ParagraphBlock::parse(pairs, input.as_span());
    }

    Ok(vec![])
}

fn parse_enclosed_block(input: Pair<Rule>) -> Result<UnimarkupBlocks, MappedLogId> {
    if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::verbatim, input.as_str()) {
        return VerbatimBlock::parse(pairs, input.as_span());
    } else if let Ok(ref mut pairs) = UnimarkupParser::parse(Rule::paragraph, input.as_str()) {
        // TODO: Add implementation for the rest of enclosed blocks, return error if none of them match
        //
        // warn and fallback to paragraph for now

        (ParserWarnLogId::UnsupportedBlock as LogId)
            .set_event_with(
                &CORE_LOG_ID_MAP,
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
                id.push(types::ELEMENT_TYPE_DELIMITER);
            }

            id.push_str(word);
        }

        id
    };

    Some(result)
}

/// Uses a custom [`pest::error::Error`] to display parsing errors using Pest's pretty print.
///
/// # Arguments
///
/// * `msg` - Custom error message
/// * `span` - Span in input Unimarkup document where this specific error occured
pub fn custom_pest_error(msg: impl Into<String>, span: pest::Span) -> String {
    use crate::frontend::parser;
    use pest::error;

    let error = error::Error::new_from_span(
        error::ErrorVariant::<parser::Rule>::CustomError {
            message: msg.into(),
        },
        span,
    );

    error.to_string()
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    #[test]
    fn test__generate_id__valid_id() {
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
    fn test__generate_id__valid_id_with_num() {
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
    fn test__generate_id__valid_id_many_symbols() {
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
    fn test__generate_id__empty_input() {
        let input = "";

        let id = super::generate_id(input);

        assert!(id.is_none(), "generate_id is some");
    }

    #[test]
    fn test__generate_id__whitespace_only() {
        let input = " ";

        let id = super::generate_id(input);

        assert!(id.is_none(), "generate_id is some");
    }
}
