use super::token_kind::TokenKind;

use crate::position::{Offset, Position};

/// Token lexed from grapheme [`Symbol`]s of the given input.
///
/// # Lifetimes
///
/// * `'input` - lifetime of input the [`Token`] was lexed from.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'input> {
    pub input: &'input str,
    pub offset: Offset,
    pub kind: TokenKind,
    pub start: Position,
    pub end: Position,
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("input", &self.input)
            .field(
                "output",
                &self.input[self.offset.start..self.offset.end].to_string(),
            )
            .field("offset", &self.offset)
            .field("kind", &self.kind)
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}
