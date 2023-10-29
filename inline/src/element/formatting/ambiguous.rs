use unimarkup_commons::scanner::position::{Offset, Position};

use crate::{
    element::Inline,
    inline_parser,
    tokenize::{
        iterator::InlineTokenIterator,
        token::{InlineToken, InlineTokenKind},
    },
};

pub fn parse(input: &mut InlineTokenIterator) -> Option<Inline> {
    let mut open_token = input.next()?;

    if input.peek_kind()?.is_space() {
        // Split ambiguous in case of leading space. Main wins
        if is_ambiguous(open_token.kind) {
            let (first_token, second_token) = split_token(open_token, main_part(open_token.kind));
            input.cache_token(second_token);
            open_token = first_token;
        } else {
            return None;
        }
    }

    if is_ambiguous(open_token.kind) {
        input.push_format(main_part(open_token.kind));
        input.push_format(sub_part(open_token.kind));
    } else {
        input.push_format(open_token.kind);
    }

    let inner = inline_parser::InlineParser::default().parse(input);

    resolve_closing(input, open_token, inner)
}

/// Tries to split an ambiguous format, so an partial open format may close on next iteration.
/// This is achieved by adapting the positions of the given ambiguous token, and caching the partial token.
pub(crate) fn ambiguous_split<'input>(
    token_iter: &mut InlineTokenIterator<'input>,
    token: &mut InlineToken<'input>,
) {
    if is_ambiguous(token.kind) {
        // Main and sub parts might be both open, but main part wins split
        // => first part will be treated as plain, and second part is cached to get a valid closing format point
        if token_iter.format_is_open(main_part(token.kind)) {
            let (first_token, second_token) = split_token(*token, sub_part(token.kind));
            token_iter.cache_token(second_token);
            *token = first_token;
        } else if token_iter.format_is_open(sub_part(token.kind)) {
            let (first_token, second_token) = split_token(*token, main_part(token.kind));
            token_iter.cache_token(second_token);
            *token = first_token;
        }
    }
}

fn resolve_closing(
    input: &mut InlineTokenIterator,
    open_token: InlineToken<'_>,
    inner: Vec<Inline>,
) -> Option<Inline> {
    let mut outer: Vec<Inline> = Vec::default();

    let updated_open = match input.peek() {
        Some(close_token) => {
            if open_token.kind == counterpart(close_token.kind)
                || open_token.kind == close_token.kind
            {
                let (attributes, end, implicit_end) = if open_token.kind == close_token.kind {
                    // consume close, because this fn call opened and now closes the format
                    input.next()?;

                    // check for optional attributes here
                    (None, close_token.end, false)
                } else {
                    // e.g. bold implicitly closed by italic close: *italic **b+i*
                    (None, close_token.start, true)
                };

                return Some(close_format(
                    input,
                    open_token,
                    inner,
                    attributes,
                    end,
                    implicit_end,
                ));
            } else if is_ambiguous(close_token.kind)
                && close_token.kind == ambiguous_part(open_token.kind)
            {
                // e.g. close = 3 stars between: **bold***italic*
                input.next()?;
                input.pop_format(open_token.kind);

                let (closing_token, cached_token) = split_token(close_token, open_token.kind);
                input.cache_token(cached_token);

                return Some(super::to_formatting(
                    open_token.kind,
                    inner,
                    None,
                    open_token.start,
                    closing_token.end,
                    false,
                ));
            } else if open_token.kind == ambiguous_part(close_token.kind) {
                // no cache, because "split" is on open token
                // e.g. close = 2 stars between: ***bold**italic*
                input.next()?;
                input.pop_format(close_token.kind);
                outer.push(super::to_formatting(
                    close_token.kind,
                    inner,
                    None, // check for optional attributes here
                    open_token.start,
                    close_token.end,
                    false,
                ));

                let (updated_open, _) = split_token(open_token, counterpart(close_token.kind));
                updated_open
            } else {
                // Close open format, but do not consume close, because close is not compatible with open one
                // This means that some outer format closed.
                // => end of this format is at start of the closing token for the outer format
                // e.g. bold close = implicit before strikethrough end: ~~strike**bold~~
                return Some(close_format(
                    input,
                    open_token,
                    inner,
                    None,
                    close_token.start,
                    true,
                ));
            }
        }
        None => {
            // close open format only and return
            // This is ok, because if ambiguous would have been split, peek() would have returned the partial closing token
            // e.g. implicit close in scoped context: [**bold]
            return Some(close_format(
                input,
                open_token,
                inner,
                None,
                input
                    .prev_token()
                    .expect("Previous token must exist, because at least the opening token came before.")
                    .end,
                true,
            ));
        }
    };

    outer.append(&mut inline_parser::InlineParser::default().parse(input));

    // Format will definitely close fully now => so remove from open formats
    input.pop_format(updated_open.kind);

    if let Some(close_token) = input.peek() {
        // open token was updated to either main or sub part from ambiguous
        if compatible(updated_open.kind, close_token.kind) {
            input.next()?;

            let (attributes, end) = if is_ambiguous(close_token.kind) {
                // ambiguous token gets split, because only part of it is used to close this open format
                // e.g. first 2 stars of the 3 at end are taken, last star is cached: ***bold + italic* bold***
                let (closing_token, cached_token) = split_token(close_token, updated_open.kind);
                input.cache_token(cached_token);
                (None, closing_token.end)
            } else {
                debug_assert!(
                    updated_open.kind == close_token.kind,
                    "Closing '{:?}' did not match updated open '{:?}'.",
                    close_token.kind,
                    updated_open.kind
                );
                // check for optional attributes here
                (None, close_token.end)
            };

            return Some(super::to_formatting(
                updated_open.kind,
                outer,
                attributes,
                updated_open.start,
                end,
                false,
            ));
        }
    }

    Some(super::to_formatting(
        updated_open.kind,
        outer,
        None,
        updated_open.start,
        input
            .prev_token()
            .expect("Previous token must exist, because at least the opening token came before.")
            .end,
        true,
    ))
}

fn close_format(
    input: &mut InlineTokenIterator<'_>,
    open_token: InlineToken<'_>,
    inner: Vec<Inline>,
    attributes: Option<Vec<Inline>>,
    end: Position,
    implicit_end: bool,
) -> Inline {
    if is_ambiguous(open_token.kind) {
        input.pop_format(main_part(open_token.kind));
        input.pop_format(sub_part(open_token.kind));

        let (outer_token, inner_token) = split_token(open_token, main_part(open_token.kind));

        super::to_formatting(
            outer_token.kind,
            vec![super::to_formatting(
                inner_token.kind,
                inner,
                None,
                inner_token.start,
                inner_token.end,
                implicit_end,
            )],
            attributes,
            outer_token.start,
            outer_token.end,
            implicit_end,
        )
    } else {
        input.pop_format(open_token.kind);
        super::to_formatting(
            open_token.kind,
            inner,
            attributes,
            open_token.start,
            end,
            implicit_end,
        )
    }
}

fn compatible(kind: InlineTokenKind, other: InlineTokenKind) -> bool {
    kind == other || kind == counterpart(other) || kind == ambiguous_part(other)
}

fn counterpart(kind: InlineTokenKind) -> InlineTokenKind {
    match kind {
        InlineTokenKind::Bold => InlineTokenKind::Italic,
        InlineTokenKind::Italic => InlineTokenKind::Bold,
        InlineTokenKind::Underline => InlineTokenKind::Subscript,
        InlineTokenKind::Subscript => InlineTokenKind::Underline,
        _ => kind,
    }
}

fn main_part(kind: InlineTokenKind) -> InlineTokenKind {
    match kind {
        InlineTokenKind::Bold => kind,
        InlineTokenKind::Italic | InlineTokenKind::BoldItalic => InlineTokenKind::Bold,
        InlineTokenKind::Underline => kind,
        InlineTokenKind::Subscript | InlineTokenKind::UnderlineSubscript => {
            InlineTokenKind::Underline
        }
        _ => kind,
    }
}

fn sub_part(kind: InlineTokenKind) -> InlineTokenKind {
    match kind {
        InlineTokenKind::Italic => kind,
        InlineTokenKind::Bold | InlineTokenKind::BoldItalic => InlineTokenKind::Italic,
        InlineTokenKind::Subscript => kind,
        InlineTokenKind::Underline | InlineTokenKind::UnderlineSubscript => {
            InlineTokenKind::Subscript
        }
        _ => kind,
    }
}

fn is_ambiguous(kind: InlineTokenKind) -> bool {
    matches!(
        kind,
        InlineTokenKind::BoldItalic | InlineTokenKind::UnderlineSubscript
    )
}

fn ambiguous_part(kind: InlineTokenKind) -> InlineTokenKind {
    match kind {
        InlineTokenKind::Italic | InlineTokenKind::Bold | InlineTokenKind::BoldItalic => {
            InlineTokenKind::BoldItalic
        }
        InlineTokenKind::Subscript
        | InlineTokenKind::Underline
        | InlineTokenKind::UnderlineSubscript => InlineTokenKind::UnderlineSubscript,
        _ => kind,
    }
}

fn split_token(
    ambiguous: InlineToken<'_>,
    first_kind: InlineTokenKind,
) -> (InlineToken<'_>, InlineToken<'_>) {
    let first_kind_len = first_kind.len();
    // It is assumed that ambiguous lenght = first token + counterpart token length
    let first_token = InlineToken {
        input: ambiguous.input,
        offset: Offset {
            start: ambiguous.offset.start,
            end: ambiguous.offset.start + first_kind_len,
        },
        kind: first_kind,
        start: ambiguous.start,
        // It is assumed that ambiguous format tokens have one code point per symbol
        end: Position {
            line: ambiguous.start.line,
            col_utf8: ambiguous.start.col_utf8 + first_kind_len,
            col_utf16: ambiguous.start.col_utf16 + first_kind_len,
            col_grapheme: ambiguous.start.col_grapheme + first_kind_len,
        },
    };
    let second_token = InlineToken {
        input: ambiguous.input,
        offset: Offset {
            start: ambiguous.offset.start + first_kind_len,
            end: ambiguous.offset.end,
        },
        kind: counterpart(first_kind),
        // It is assumed that ambiguous format tokens have one code point per symbol
        start: Position {
            line: ambiguous.start.line,
            col_utf8: ambiguous.start.col_utf8 + first_kind_len,
            col_utf16: ambiguous.start.col_utf16 + first_kind_len,
            col_grapheme: ambiguous.start.col_grapheme + first_kind_len,
        },
        end: ambiguous.end,
    };

    (first_token, second_token)
}

// #[cfg(test)]
// mod test {
//     use unimarkup_commons::scanner::token::iterator::TokenIterator;

//     use crate::element::{
//         formatting::{Subscript, Underline},
//         plain::Plain,
//     };

//     use super::*;

//     #[test]
//     fn parse_underline_subscript() {
//         let symbols = unimarkup_commons::scanner::scan_str("___underline__subscript_");
//         let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

//         let inline = parse(&mut token_iter).unwrap();

//         assert_eq!(
//             inline,
//             Subscript {
//                 inner: vec![
//                     Underline {
//                         inner: vec![Plain {
//                             content: "underline".to_string(),
//                         }
//                         .into()]
//                     }
//                     .into(),
//                     Plain {
//                         content: "subscript".to_string(),
//                     }
//                     .into()
//                 ],
//             }
//             .into(),
//             "Subscript + underline not correctly parsed."
//         )
//     }

//     #[test]
//     fn parse_underline_subscript_ambiguous_close() {
//         let symbols = unimarkup_commons::scanner::scan_str("__underline_subscript___");
//         let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

//         let inline = parse(&mut token_iter).unwrap();

//         assert_eq!(
//             inline,
//             Underline {
//                 inner: vec![
//                     Plain {
//                         content: "underline".to_string(),
//                     }
//                     .into(),
//                     Subscript {
//                         inner: vec![Plain {
//                             content: "subscript".to_string(),
//                         }
//                         .into()]
//                     }
//                     .into(),
//                 ],
//             }
//             .into(),
//             "Subscript + underline not correctly parsed."
//         )
//     }
// }
