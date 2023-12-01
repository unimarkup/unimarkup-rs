use std::marker::PhantomData;

use crate::{lexer::token::iterator::TokenIterator, parsing::Parser};

pub struct LogicContext {}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicAst(String); // TODO: setup AST

pub struct LogicParser<'slice, 'input, P, T>
where
    P: Parser<T>,
{
    /// The iterator over [`Token`](crate::lexer::token::Token)s of Unimarkup content.
    pub iter: TokenIterator<'slice, 'input>,
    /// The parser to use to parse Unimarkup content inside logic elements.
    pub um_parser: P,
    /// Context for logic element parsing.
    pub context: LogicContext,
    phantom: PhantomData<T>,
}
