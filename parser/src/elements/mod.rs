//! Available elements for a Unimarkup document.

pub mod atomic;
pub mod blocks;
pub mod enclosed;
pub mod indents;
pub mod kind;
pub mod preamble;

use unimarkup_commons::{
    lexer::{position::Position, span::Span},
    parsing::Element,
};

use self::blocks::Block;

/// Type alias for a vector of [`Block`] enum.
pub type Blocks = Vec<Block>;

/// Needed trait to implement [`Element`] trait for Vec<Block> in this crate
pub trait BlockElement {
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

impl Element for dyn BlockElement {
    fn as_unimarkup(&self) -> String {
        self.as_unimarkup()
    }

    fn start(&self) -> Position {
        self.start()
    }

    fn end(&self) -> Position {
        self.end()
    }
}
