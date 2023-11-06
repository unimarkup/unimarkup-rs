//! Contains the [`Element`] trait every Unimarkup element must implement.

use crate::lexer::{position::Position, span::Span};

pub trait Element {
    fn to_plain_string(&self) -> String;
    fn start(&self) -> Position;
    fn end(&self) -> Position;
    fn span(&self) -> Span {
        Span {
            start: self.start(),
            end: self.end(),
        }
    }
}
