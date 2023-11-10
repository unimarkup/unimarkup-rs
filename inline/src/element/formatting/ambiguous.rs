//! Contains the parser for ambiguous formats like [`Bold`](super::Bold) and [`Italic`](super::Italic), or [`Underline`](super::Underline) and [`Subscript`](super::Subscript).

use unimarkup_commons::lexer::{
    position::{Offset, Position},
    PeekingNext,
};

use crate::{
    element::Inline,
    parser::InlineParser,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind, InlineToken},
};

/// Parses and creates an ambiguous format.
pub(crate) fn parse<'s, 'i>(
    mut parser: InlineParser<'s, 'i>,
) -> (InlineParser<'s, 'i>, Option<Inline>) {
    let open_token_opt = parser.iter.peeking_next(|_| true);
    if open_token_opt.is_none() {
        return (parser, None);
    }

    let mut open_token = open_token_opt.expect("Checked above to be not None.");

    if parser.iter.peek_kind().map_or(true, |t| t.is_space()) {
        // Split ambiguous in case of leading space. Main wins
        if is_ambiguous(open_token.kind) {
            parser.iter.next(); // consume open token before split

            let (first_token, second_token) = split_token(open_token, main_part(open_token.kind));
            parser.iter.cache_token(second_token);
            open_token = first_token;
        } else {
            return (parser, None);
        }
    } else {
        parser.iter.next(); // consume open token => now it will lead to Some(inline)
    }

    if is_ambiguous(open_token.kind) {
        parser.iter.open_format(&main_part(open_token.kind));
        parser.iter.open_format(&sub_part(open_token.kind));
    } else {
        parser.iter.open_format(&open_token.kind);
    }

    let (updated_parser, inner) = InlineParser::parse(parser);
    parser = updated_parser;

    resolve_closing(parser, open_token, inner)
}

/// Tries to split an ambiguous format, so an partial open format may close on next iteration.
/// This is achieved by adapting the positions of the given ambiguous token, and caching the partial token.
pub(crate) fn ambiguous_split<'input>(
    token_iter: &mut InlineTokenIterator<'_, 'input>,
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

/// Resolves closing of the ambiguous format.
/// Either by fully closing the format, or splitting the closing token, and parsing the second half until this can be closed aswell.
fn resolve_closing<'slice, 'input>(
    mut parser: InlineParser<'slice, 'input>,
    open_token: InlineToken<'input>,
    inner: Vec<Inline>,
) -> (InlineParser<'slice, 'input>, Option<Inline>) {
    let mut outer: Vec<Inline> = Vec::default();

    let updated_open = match parser.iter.peek() {
        Some(close_token) => {
            if open_token.kind == counterpart(close_token.kind)
                || open_token.kind == close_token.kind
            {
                let (attributes, end, implicit_end) = if open_token.kind == close_token.kind {
                    // consume close, because this fn call opened and now closes the format
                    parser
                        .iter
                        .next()
                        .expect("Peeked before, so `next` must return Some.");

                    // check for optional attributes here
                    (None, close_token.end, false)
                } else {
                    // e.g. bold implicitly closed by italic close: *italic **b+i*
                    (None, close_token.start, true)
                };

                let inline = to_inline(
                    open_token,
                    &mut parser.iter,
                    inner,
                    attributes,
                    end,
                    implicit_end,
                );
                return (parser, Some(inline));
            } else if is_ambiguous(close_token.kind)
                && close_token.kind == ambiguous_part(open_token.kind)
            {
                // e.g. close = 3 stars between: **bold***italic*
                parser
                    .iter
                    .next()
                    .expect("Peeked before, so `next` must return Some.");
                parser.iter.close_format(&open_token.kind);

                let (closing_token, cached_token) = split_token(close_token, open_token.kind);
                parser.iter.cache_token(cached_token);

                return (
                    parser,
                    Some(super::to_formatting(
                        open_token.kind,
                        inner,
                        None,
                        open_token.start,
                        closing_token.end,
                        false,
                    )),
                );
            } else if open_token.kind == ambiguous_part(close_token.kind) {
                // no cache, because "split" is on open token
                // e.g. close = 2 stars between: ***bold**italic*
                parser
                    .iter
                    .next()
                    .expect("Peeked before, so `next` must return Some.");
                parser.iter.close_format(&close_token.kind);
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
                let inline = to_inline(
                    open_token,
                    &mut parser.iter,
                    inner,
                    None,
                    close_token.start,
                    true,
                );
                return (parser, Some(inline));
            }
        }
        None => {
            // close open format only and return
            // This is ok, because if ambiguous would have been split, peek() would have returned the partial closing token
            // e.g. implicit close in scoped context: [**bold]
            let prev_token = parser.iter.prev_token().expect(
                "Previous token must exist, because at least the opening token came before.",
            );
            let inline = to_inline(
                open_token,
                &mut parser.iter,
                inner,
                None,
                crate::element::helper::implicit_end_using_prev(&prev_token),
                true,
            );
            return (parser, Some(inline));
        }
    };

    let (updated_parser, mut inlines) = InlineParser::parse(parser);
    parser = updated_parser;
    outer.append(&mut inlines);

    // Format will definitely close fully now => so remove from open formats
    parser.iter.close_format(&updated_open.kind);

    if let Some(close_token) = parser.iter.peek() {
        // open token was updated to either main or sub part from ambiguous
        if compatible(updated_open.kind, close_token.kind) {
            parser
                .iter
                .next()
                .expect("Peeked before, so `next` must return Some.");

            let (attributes, end) = if is_ambiguous(close_token.kind) {
                // ambiguous token gets split, because only part of it is used to close this open format
                // e.g. first 2 stars of the 3 at end are taken, last star is cached: ***bold + italic* bold***
                let (closing_token, cached_token) = split_token(close_token, updated_open.kind);
                parser.iter.cache_token(cached_token);
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

            return (
                parser,
                Some(super::to_formatting(
                    updated_open.kind,
                    outer,
                    attributes,
                    updated_open.start,
                    end,
                    false,
                )),
            );
        }
    }

    let prev_token = parser
        .iter
        .prev_token()
        .expect("Previous token must exist, because at least the opening token came before.");
    (
        parser,
        Some(super::to_formatting(
            updated_open.kind,
            outer,
            None,
            updated_open.start,
            crate::element::helper::implicit_end_using_prev(&prev_token),
            true,
        )),
    )
}

/// Converts the ambiguous format into its inline element.
fn to_inline<'input>(
    open_token: InlineToken<'input>,
    input: &mut InlineTokenIterator<'_, 'input>,
    inner: Vec<Inline>,
    attributes: Option<Vec<Inline>>,
    end: Position,
    implicit_end: bool,
) -> Inline {
    if is_ambiguous(open_token.kind) {
        input.close_format(&main_part(open_token.kind));
        input.close_format(&sub_part(open_token.kind));

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
        input.close_format(&open_token.kind);
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

/// Returns `true` if the two kinds are compatible.
/// Meaning they both share the same keyword.
fn compatible(kind: InlineTokenKind, other: InlineTokenKind) -> bool {
    kind == other || kind == counterpart(other) || kind == ambiguous_part(other)
}

/// Returns the counterpart of the given kind.
/// e.g. `italic` for `bold`
///
/// The ambiguous variants have no counterpart.
fn counterpart(kind: InlineTokenKind) -> InlineTokenKind {
    match kind {
        InlineTokenKind::Bold => InlineTokenKind::Italic,
        InlineTokenKind::Italic => InlineTokenKind::Bold,
        InlineTokenKind::Underline => InlineTokenKind::Subscript,
        InlineTokenKind::Subscript => InlineTokenKind::Underline,
        _ => kind,
    }
}

/// Returns the main part of an ambiguous kind.
/// e.g. `bold` for bold/italic
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

/// Returns the sub part of an ambiguous kind.
/// e.g. `italic` for bold/italic
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

/// Returns `true` if the given kind is ambiguous.
/// e.g. `bolditalic`
pub(crate) fn is_ambiguous(kind: InlineTokenKind) -> bool {
    matches!(
        kind,
        InlineTokenKind::BoldItalic | InlineTokenKind::UnderlineSubscript
    )
}

/// Returns the ambiguous kind for the given kind.
/// e.g. `bolditalic` for bold/italic
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

/// Splits an ambiguous token using the given kind as the first part.
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
