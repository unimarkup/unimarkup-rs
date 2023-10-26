mod kind;
pub use kind::*;

use super::{
    position::{Offset, Position},
    Symbol,
};

pub mod iterator;

/// Token lexed from Unimarkup text.
///
/// # Lifetimes
///
/// * `'input` - lifetime of input the [`Token`] was lexed from.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'input> {
    pub input: &'input str,
    pub offset: Offset,
    pub kind: TokenKind,
    pub start: Position,
    pub end: Position,
}

impl<'input> From<&Symbol<'input>> for Token<'input> {
    fn from(value: &Symbol<'input>) -> Self {
        Token {
            input: value.input,
            offset: value.offset,
            kind: TokenKind::from(value.kind),
            start: value.start,
            end: value.end,
        }
    }
}
