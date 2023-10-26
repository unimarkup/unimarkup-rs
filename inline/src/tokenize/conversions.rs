use crate::element::{
    plain::{EscapedPlain, Plain},
    spaces::{EscapedNewline, EscapedWhitespace, Newline, Whitespace},
    Inline,
};

use super::token::{InlineToken, InlineTokenKind};

impl<'input> From<InlineToken<'input>> for Inline {
    fn from(value: InlineToken<'input>) -> Self {
        match value.kind {
            InlineTokenKind::Newline => Inline::Newline(Newline {}),
            InlineTokenKind::EscapedNewline => Inline::EscapedNewline(EscapedNewline {}),
            InlineTokenKind::Whitespace => Inline::Whitespace(Whitespace {}),
            InlineTokenKind::EscapedWhitespace => Inline::EscapedWhitespace(EscapedWhitespace {
                space: value.as_str().to_string(),
            }),
            InlineTokenKind::EscapedPlain => Inline::EscapedPlain(EscapedPlain {
                content: value.as_str().to_string(),
            }),

            // All other tokens are either created in parser, or did not resolve to an element => take as plain
            _ => Inline::Plain(Plain {
                content: value.as_str().to_string(),
            }),
        }
    }
}
