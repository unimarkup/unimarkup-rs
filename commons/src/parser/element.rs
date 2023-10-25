use crate::scanner::span::Span;

use super::Parser;

pub trait Element<P>: Parser<P> {
    fn start(&self) -> Span;
    fn end(&self) -> Span;
}
