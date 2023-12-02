use std::marker::PhantomData;

use itertools::{Itertools, PeekingNext};

use crate::{
    lexer::{
        position::Position,
        symbol::SymbolKind,
        token::{iterator::TokenIterator, TokenKind},
    },
    parsing::{Element, Parser},
};

pub struct LogicContext {}

// TODO: setup AST
#[derive(Debug, PartialEq, Clone)]
pub struct LogicAst {
    inner: String,
    implicit_closed: bool,
    start: Position,
    end: Position,
}

impl Element for LogicAst {
    fn as_unimarkup(&self) -> String {
        // Note: Closing `}` is already part of `inner`.
        format!("{{${}", self.inner,)
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

pub struct LogicParser<'slice, 'input, P, T, C>
where
    // The parser to use to parse Unimarkup content inside logic elements.
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    pub iter: TokenIterator<'slice, 'input>,
    /// Context for logic element parsing.
    pub context: LogicContext,
    um_parser_context: Option<C>,
    um_parser: PhantomData<P>,
    um_parser_ok_result: PhantomData<T>,
}

impl<'slice, 'input, P, T, C> Parser<'slice, 'input, LogicAst, LogicContext>
    for LogicParser<'slice, 'input, P, T, C>
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    fn new(iter: TokenIterator<'slice, 'input>, context: LogicContext) -> Self {
        Self {
            iter,
            context,
            um_parser_context: None,
            um_parser: PhantomData,
            um_parser_ok_result: PhantomData,
        }
    }

    fn parse(mut self) -> (Self, Option<LogicAst>) {
        // TODO: implement logic parsing
        // The following code is more like a placeholder that takes logic elements as plain content.
        // It only considers scopes for `{}`. Excluding possible scopes from `"`, `'`, "`".

        let next_token_opt = self.iter.peeking_next(|_| true);
        let open_brace = match next_token_opt {
            Some(token) if token.kind == TokenKind::OpenBrace => token,
            Some(_) | None => return (self, None),
        };

        let dollar_token = match self.iter.peeking_next(|_| true) {
            Some(token) if token.kind == TokenKind::Dollar(1) => token,
            Some(_) | None => return (self, None),
        };

        let mut scope: usize = 0;
        let mut inner = String::new();
        let mut end = dollar_token.end;

        while scope > 0 && self.iter.peek().is_some() {
            let next_tokens = self.iter.peeking_take_while(|t| {
                !matches!(t.kind, TokenKind::OpenBrace | TokenKind::CloseBrace)
            });
            for t in next_tokens {
                inner.push_str(&String::from(t));
                end = t.end;
            }

            match self.iter.peeking_next(|_| true) {
                Some(t) if t.kind == TokenKind::OpenBrace => {
                    inner.push_str(SymbolKind::OpenBrace.as_str());
                    end = t.end;
                    scope += 1;
                }
                Some(t) if t.kind == TokenKind::CloseBrace => {
                    inner.push_str(SymbolKind::CloseBrace.as_str());
                    end = t.end;
                    scope -= 1;
                }
                Some(_) => {
                    debug_assert!(false, "`peeking_take_while() only allows open/close brace.");
                }
                None => {}
            }
        }

        self.iter.skip_to_peek();

        (
            self,
            Some(LogicAst {
                inner,
                implicit_closed: scope > 0,
                start: open_brace.start,
                end,
            }),
        )
    }

    fn context(&self) -> &LogicContext {
        &self.context
    }

    fn context_mut(&mut self) -> &mut LogicContext {
        &mut self.context
    }

    fn iter(&mut self) -> &mut TokenIterator<'slice, 'input> {
        &mut self.iter
    }

    fn into_inner(self) -> (TokenIterator<'slice, 'input>, LogicContext) {
        (self.iter, self.context)
    }
}

impl<'slice, 'input, P, T, C> LogicParser<'slice, 'input, P, T, C>
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    pub fn um_parser(self) -> P {
        P::new(self.iter, self.um_parser_context.unwrap_or_default())
    }

    pub fn with_um_context(
        iter: TokenIterator<'slice, 'input>,
        context: LogicContext,
        um_context: C,
    ) -> Self {
        Self {
            iter,
            context,
            um_parser_context: Some(um_context),
            um_parser: PhantomData,
            um_parser_ok_result: PhantomData,
        }
    }
}
