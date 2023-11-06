//! Available elements for a Unimarkup document.

pub mod atomic;
pub mod blocks;
pub mod enclosed;
pub mod indents;
pub mod kind;
pub mod preamble;
pub mod types;

use unimarkup_commons::{
    lexer::{position::Position, span::Span},
    parsing::Element,
};

use self::blocks::Block;

/// Type alias for a vector of [`Block`] enum.
pub type Blocks = Vec<Block>;

// Needed to implement trait for Vec<Block> in this crate
pub trait BlockElement {
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
