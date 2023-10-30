use unimarkup_commons::lexer::position::Position;

use crate::tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind, InlineToken};

pub(super) fn implicit_end_using_prev(prev_token: &InlineToken<'_>) -> Position {
    match prev_token.kind {
        // To prevent implicit closed elements from spaning over the closing line end.
        InlineTokenKind::EscapedNewline | InlineTokenKind::Newline | InlineTokenKind::Eoi => {
            prev_token.start
        }
        _ => prev_token.end,
    }
}
