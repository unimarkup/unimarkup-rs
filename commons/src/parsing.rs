//! Contains the [`Element`] trait every Unimarkup element must implement.

use itertools::Itertools;

use crate::lexer::{
    position::Position, span::Span, symbol::SymbolKind, token::iterator::TokenIterator,
};

/// Every Unimarkup element must implement this trait.
pub trait Element {
    /// Shows the element in its original plain markup form.
    fn as_unimarkup(&self) -> String;
    /// Return the start of the element in the original content.
    fn start(&self) -> Position;
    /// Return the end of the element in the original content.
    fn end(&self) -> Position;
    /// The span of an element in the original content.
    fn span(&self) -> Span {
        Span {
            start: self.start(),
            end: self.end(),
        }
    }
}

pub trait Parser<'slice, 'input, T, C>
where
    Self: std::marker::Sized,
    T: std::marker::Sized + Element,
{
    fn new(iter: TokenIterator<'slice, 'input>, context: C) -> Self;
    fn parse(self) -> (Self, Result<T, ParserError>);
    fn context(&self) -> &C;
    fn context_mut(&mut self) -> &mut C;
    fn iter(&mut self) -> &mut TokenIterator<'slice, 'input>;
    fn into_inner(self) -> (TokenIterator<'slice, 'input>, C);
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserError {
    /// Syntax violation was detected while parsing a Unimarkup element.
    /// Meaning that no other element except paragraph and plain would match.
    /// Therefore, the element span must be converted to an [`InvalidContent`] element.
    SyntaxViolation,
    /// The first tokens did not match the required start sequence of the element.
    /// Another element type should be tried for the same tokens.
    InvalidStart,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidContent(Vec<InvalidLine>);

impl Element for InvalidContent {
    fn as_unimarkup(&self) -> String {
        self.0.iter().join(SymbolKind::Newline.as_str())
    }

    fn start(&self) -> Position {
        self.0.first().map(|i| i.start).unwrap_or_default()
    }

    fn end(&self) -> Position {
        self.0.last().map(|i| i.end).unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidLine {
    content: String,
    start: Position,
    end: Position,
}

impl Element for InvalidLine {
    fn as_unimarkup(&self) -> String {
        self.content.clone()
    }

    fn start(&self) -> Position {
        self.start
    }

    fn end(&self) -> Position {
        self.end
    }
}

impl std::fmt::Display for InvalidLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
