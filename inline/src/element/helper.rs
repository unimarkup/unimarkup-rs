//! Contains helper functions for inline parsing.

use unimarkup_commons::lexer::position::Position;

use crate::tokenize::{kind::InlineTokenKind, InlineToken};

/// Returns the end position for implicitly closed elements.
pub(super) fn implicit_end_using_prev(prev_token: &InlineToken<'_>) -> Position {
    match prev_token.kind {
        // To prevent implicit closed elements from spaning over the closing line end.
        InlineTokenKind::EscapedNewline | InlineTokenKind::Newline | InlineTokenKind::Eoi => {
            prev_token.start
        }
        _ => prev_token.end,
    }
}
