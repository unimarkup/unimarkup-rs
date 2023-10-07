//! Module for parsing of Unimarkup elements.

use logid::log;
use unimarkup_commons::scanner::{EndMatcher, SymbolIterator, SymbolKind};

use crate::{
    document::Document,
    elements::{
        atomic::{Heading, Paragraph},
        enclosed::Verbatim,
        indents::BulletList,
        Blocks,
    },
    log_id::MainParserInfo,
    metadata::{Metadata, MetadataKind},
    security,
};
use unimarkup_commons::config::Config;

/// Parser as function that can parse Unimarkup content
pub type ParserFn = for<'i> fn(&mut SymbolIterator<'i>) -> Option<Blocks>;

/// Output of symbol tokenization by a parser of a block.
pub(crate) struct TokenizeOutput<T> {
    pub(crate) tokens: Vec<T>,
}

/// Trait implemented by a parser for each Unimarkup element.
pub(crate) trait ElementParser {
    /// Token type produced by tokenization.
    type Token<'a>;

    /// Function that converts input symbols into tokens specific for the given element.
    fn tokenize<'i>(input: &mut SymbolIterator<'i>) -> Option<TokenizeOutput<Self::Token<'i>>>;

    /// Function that parses tokenization output and produces one or more Unimarkup elements.
    fn parse(input: Vec<Self::Token<'_>>) -> Option<Blocks>;
}

// Makes it impossible to implement `ParserGenerator` trait outside of this module,
// but still makes it possible to name `ParserGenerator` and use it as a bound.
mod private {
    pub trait Sealed {}
    impl<'a, T> Sealed for T where T: super::ElementParser + 'a + 'static {}
}

/// Trait implemented by all Unimarkup elements that can generate parser function for their
/// content.
pub trait ParserGenerator: private::Sealed {
    /// Generates parser function for the given Unimarkup element.
    fn generate_parser() -> ParserFn;
}

impl<'a, T> ParserGenerator for T
where
    T: ElementParser + 'a + 'static,
{
    // NOTE: we might need some context information for parsers. An option could be to pass
    // some kind of Context struct into generate_parser and use that for whatever we need to.
    fn generate_parser() -> ParserFn {
        |input| {
            let tokenize_output = T::tokenize(input)?;
            let blocks = T::parse(tokenize_output.tokens)?;

            Some(blocks)
        }
    }
}

/// Parser of unimarkup content.
#[derive(Clone)]
pub struct MainParser {
    parsers: Vec<ParserFn>,
    default_parser: ParserFn,
}

impl Default for MainParser {
    fn default() -> Self {
        log!(MainParserInfo::StartInitializing);

        let default = Paragraph::generate_parser();

        let mut parser = Self {
            parsers: Vec::with_capacity(2),
            default_parser: default,
        };

        // TODO: how to handle preamble parser?
        parser.register_parser(Heading::generate_parser());
        parser.register_parser(Verbatim::generate_parser());
        parser.register_parser(BulletList::generate_parser());

        log!(MainParserInfo::Initialized);
        parser
    }
}

impl MainParser {
    fn register_parser(&mut self, parser: ParserFn) {
        self.parsers.push(parser);
    }

    /// Parses Unimarkup content and produces Unimarkup blocks.
    pub fn parse(&self, mut input: &mut SymbolIterator) -> Blocks {
        let mut blocks = Vec::default();

        #[cfg(debug_assertions)]
        let mut curr_len = input.max_len();

        input.reset_peek();

        'outer: while let Some(kind) = input.peek_kind() {
            match kind {
                // skip newlines and empty lines before elements
                SymbolKind::Newline => {
                    while input.consumed_is_empty_line() {
                        // consume contiguous empty lines
                    }
                    // Consume newline before next block element
                    if input.next().is_none() {
                        break 'outer;
                    }
                }

                // stop parsing when end of input is reached
                SymbolKind::EOI => break 'outer,

                // no parser will match, parse with default parser
                _ if kind.is_not_keyword() => match (self.default_parser)(input) {
                    Some(mut default) => blocks.append(&mut default),
                    None => break 'outer,
                },

                // symbol is start of a block, some parser should match
                _ => {
                    for parser_fn in &self.parsers {
                        let mut iter = input.clone();
                        if let Some(mut res_blocks) = parser_fn(&mut iter) {
                            blocks.append(&mut res_blocks);
                            *input = iter;
                            continue 'outer; // start from first parser on next input
                        }
                    }

                    // no registered parser matched -> use default parser
                    match (self.default_parser)(input) {
                        Some(mut default) => blocks.append(&mut default),
                        None => break 'outer,
                    }
                }
            }

            #[cfg(debug_assertions)]
            {
                assert!(
                    input.max_len() < curr_len,
                    "Parser consumed no symbol in iteration."
                );
                curr_len = input.max_len();
            }
        }

        blocks
    }
}

/// Parses and returns a Unimarkup document.
pub fn parse_unimarkup(um_content: &str, config: &mut Config) -> Document {
    let parser = MainParser::default();

    let symbols = unimarkup_commons::scanner::scan_str(um_content);
    let mut symbols_iter = SymbolIterator::from(&*symbols);
    let blocks = parser.parse(&mut symbols_iter);

    let mut unimarkup = Document {
        config: config.clone(),
        blocks,
        ..Default::default()
    };

    let metadata = Metadata {
        file: config.input.clone(),
        contenthash: security::get_contenthash(um_content),
        preamble: String::new(),
        kind: MetadataKind::Root,
        namespace: ".".to_string(),
    };

    unimarkup.metadata.push(metadata);

    unimarkup
}
