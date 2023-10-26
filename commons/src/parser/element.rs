use crate::scanner::{position::Position, span::Span};

use super::Parser;

pub trait Element<P>: Parser<P> {
    fn span(&self) -> Span;
    fn start(&self) -> Position;
    fn end(&self) -> Position;
}
