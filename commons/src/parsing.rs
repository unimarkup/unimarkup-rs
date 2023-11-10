//! Contains the [`Element`] trait every Unimarkup element must implement.

use crate::lexer::{position::Position, span::Span};

/// Every Unimarkup element must implement this trait.
pub trait Element {
    /// Convert the element into its original plain markup form.
    fn to_plain_string(&self) -> String;
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
