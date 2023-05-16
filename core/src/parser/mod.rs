//! Module for parsing of Unimarkup elements.

use logid::capturing::MappedLogId;
use unimarkup_commons::scanner::{IntoSymbols, Symbol, SymbolKind};

use crate::{
    document::Document,
    elements::{
        atomic::{Heading, Paragraph},
        enclosed::Verbatim,
        Blocks,
    },
    metadata::{Metadata, MetadataKind},
    security,
};
use unimarkup_commons::config::Config;

/// Parser as function that can parse Unimarkup content
pub type ParserFn = for<'i> fn(&'i [Symbol<'i>]) -> Option<(Blocks, &'i [Symbol<'i>])>;

/// Output of symbol tokenization by a parser of a block.
pub(crate) struct TokenizeOutput<'a, T>
where
    T: 'a,
{
    pub(crate) tokens: Vec<T>,
    pub(crate) rest_of_input: &'a [Symbol<'a>],
}

/// Trait implemented by a parser for each Unimarkup element.
pub(crate) trait ElementParser {
    /// Token type produced by tokenization.
    type Token<'a>;

    /// Function that converts input symbols into tokens specific for the given element.
    fn tokenize<'i>(input: &'i [Symbol<'i>]) -> Option<TokenizeOutput<'i, Self::Token<'i>>>;

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

            Some((blocks, tokenize_output.rest_of_input))
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
        tracing::info!("Initializing MainParser");

        let default = Paragraph::generate_parser();

        let mut parser = Self {
            parsers: Vec::with_capacity(2),
            default_parser: default,
        };

        // TODO: how to handle preamble parser?
        parser.register_parser(Heading::generate_parser());
        parser.register_parser(Verbatim::generate_parser());

        tracing::info!("MainParser initialized");
        parser
    }
}

impl MainParser {
    fn register_parser(&mut self, parser: ParserFn) {
        self.parsers.push(parser);
    }

    /// Parses Unimarkup content and produces Unimarkup blocks.
    pub fn parse<'s>(&self, input: impl IntoSymbols<'s, &'s [Symbol<'s>]>) -> Blocks {
        let mut input = input.into_symbols();
        let mut blocks = Vec::default();

        #[cfg(debug_assertions)]
        let mut input_len = input.len();

        'outer: while let Some(sym) = input.first() {
            match sym.kind {
                // skip blanklines
                SymbolKind::Blankline => input = &input[1..],

                // stop parsing when end of input is reached
                SymbolKind::EOI => break,

                // no parser will match, parse with default parser
                _ if sym.is_not_keyword() => {
                    let (mut res_blocks, rest_of_input) = (self.default_parser)(input)
                        .expect("Default parser could not parse content!");

                    blocks.append(&mut res_blocks);
                    input = rest_of_input;
                }

                // symbol is start of a block, some parser should match
                _ => {
                    for parser_fn in &self.parsers {
                        if let Some((mut res_blocks, rest_of_input)) = parser_fn(input) {
                            blocks.append(&mut res_blocks);
                            input = rest_of_input;
                            continue 'outer; // start from first parser on next input
                        }
                    }

                    // no registered parser matched -> use default parser
                    let (mut res_blocks, rest_of_input) = (self.default_parser)(input)
                        .expect("Default parser could not parse content!");

                    blocks.append(&mut res_blocks);
                    input = rest_of_input;
                }
            }

            #[cfg(debug_assertions)]
            {
                assert_ne!(input.len(), input_len);
                input_len = input.len();
            }
        }

        blocks
    }
}

/// Parses and returns a Unimarkup document.
pub fn parse_unimarkup(um_content: &str, config: &mut Config) -> Result<Document, MappedLogId> {
    let parser = MainParser::default();

    let symbols = um_content.into_symbols();

    let blocks = parser.parse(&symbols);

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

    Ok(unimarkup)
}
