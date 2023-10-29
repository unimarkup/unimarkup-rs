mod element;
mod generator;

pub use element::*;
pub use generator::*;

use crate::lexer::SymbolIterator;

/// Parser as function that can parse Unimarkup content
pub type ParserFn<T> = for<'i> fn(&mut SymbolIterator<'i>) -> Option<T>;

/// Trait implemented by a parser for each Unimarkup element.
pub trait Parser<T> {
    /// Function that parses tokenization output and produces one or more Unimarkup elements.
    fn parse(input: &mut SymbolIterator) -> Option<T>;
}

pub trait GroupParser<T>: Default {
    fn register_parser(&mut self, parser: ParserFn<T>);

    fn parse(&self, input: &mut SymbolIterator) -> Vec<T>;
}
