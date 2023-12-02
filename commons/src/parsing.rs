//! Contains the [`Element`] trait every Unimarkup element must implement.

use crate::lexer::{position::Position, span::Span, token::iterator::TokenIterator};

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
    fn parse(self) -> (Self, Option<T>);
    fn context(&self) -> &C;
    fn context_mut(&mut self) -> &mut C;
    fn iter(&mut self) -> &mut TokenIterator<'slice, 'input>;
    fn into_inner(self) -> (TokenIterator<'slice, 'input>, C);
}
