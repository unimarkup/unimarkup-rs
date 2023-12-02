use std::marker::PhantomData;

use crate::{
    lexer::token::iterator::TokenIterator,
    parsing::{Element, Parser},
};

use super::token::AttributeToken;

pub struct AttributeContext {}

pub struct AttributeTokenizer<'slice, 'input, P, T, C>
where
    // The parser to use to parse Unimarkup content inside logic elements.
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    pub iter: TokenIterator<'slice, 'input>,
    /// Context for attribute tokenization.
    pub context: AttributeContext,
    um_parser_context: Option<C>,
    um_parser: PhantomData<P>,
    um_parser_ok_result: PhantomData<T>,
}

impl Element for Vec<AttributeToken> {
    fn as_unimarkup(&self) -> String {
        self.iter().fold(String::new(), |mut s, t| {
            s.push_str(&t.as_unimarkup());
            s
        })
    }

    fn start(&self) -> crate::lexer::position::Position {
        self.first().map(|t| t.start()).unwrap_or_default()
    }

    fn end(&self) -> crate::lexer::position::Position {
        self.last().map(|t| t.end()).unwrap_or_default()
    }
}

impl<'slice, 'input, P, T, C> Parser<'slice, 'input, Vec<AttributeToken>, AttributeContext>
    for AttributeTokenizer<'slice, 'input, P, T, C>
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    fn new(iter: TokenIterator<'slice, 'input>, context: AttributeContext) -> Self {
        Self {
            iter,
            context,
            um_parser_context: None,
            um_parser: PhantomData,
            um_parser_ok_result: PhantomData,
        }
    }

    fn parse(self) -> (Self, Option<Vec<AttributeToken>>) {
        todo!()
    }

    fn context(&self) -> &AttributeContext {
        &self.context
    }

    fn context_mut(&mut self) -> &mut AttributeContext {
        &mut self.context
    }

    fn iter(&mut self) -> &mut TokenIterator<'slice, 'input> {
        &mut self.iter
    }

    fn into_inner(self) -> (TokenIterator<'slice, 'input>, AttributeContext) {
        (self.iter, self.context)
    }
}
