//! Module for parsing of Unimarkup elements.

mod symbol;

use symbol::Symbol;

use crate::elements::Blocks;

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

impl<'a, T: 'a> TokenizeOutput<'a, T> {
    pub fn tokens(&self) -> &[T] {
        &self.tokens
    }

    pub fn rest(&self) -> &[Symbol] {
        self.rest_of_input
    }
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
