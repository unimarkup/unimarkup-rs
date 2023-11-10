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
    /// Converts a block into the original string representation.
    /// e.g. "# Heading" for level-1 heading
    fn to_plain_string(&self) -> String;
    /// The start of a block in the original content.
    fn start(&self) -> Position;
    /// The end of a block in the original content.
    fn end(&self) -> Position;
    /// The span of the block in the original content.
    fn span(&self) -> Span {
        Span {
            start: self.start(),
            end: self.end(),
        }
    }
}

impl Element for dyn BlockElement {
    fn to_plain_string(&self) -> String {
        self.to_plain_string()
    }

    fn start(&self) -> Position {
        self.start()
    }

    fn end(&self) -> Position {
        self.end()
    }
}
