mod kind;
pub use kind::*;

use super::{
    position::{Offset, Position},
    Symbol, SymbolKind,
};

pub mod implicit;
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

impl From<Token<'_>> for String {
    fn from(value: Token<'_>) -> Self {
        match value.kind {
            TokenKind::Plain | TokenKind::Punctuation | TokenKind::Whitespace => {
                value.input[value.offset.start..value.offset.end].to_string()
            }
            TokenKind::EscapedPlain | TokenKind::EscapedWhitespace => {
                let escaped_str = &value.input
                    [(value.offset.start + SymbolKind::Backslash.as_str().len())..value.offset.end];
                let mut s = String::with_capacity(escaped_str.len());
                s.push_str(escaped_str);
                s
            }
            _ => String::from(value.kind),
        }
    }
}
