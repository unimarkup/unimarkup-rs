use unimarkup_commons::scanner::position::Position;

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
            let mut cached = open_token;
            cached.kind = sub_part(open_token.kind);

            //TODO: update positions

            input.cache_token(cached);
            open_token.kind = main_part(open_token.kind);
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

fn resolve_closing(
    input: &mut InlineTokenIterator,
    mut open_token: InlineToken<'_>,
    inner: Vec<Inline>,
) -> Option<Inline> {
    let mut outer: Vec<Inline> = Vec::default();

    let updated_open = match input.peek() {
        Some(close_token) => {
            if open_token.kind == counterpart(close_token.kind)
                || open_token.kind == close_token.kind
            {
                if open_token.kind == close_token.kind {
                    // consume close, because this fn call opened and now closes the format
                    input.next()?;

                    // check for optional attributes here
                }

                return Some(close_format(input, open_token, inner, close_token.end));
            } else if !is_ambiguous(open_token.kind)
                && open_token.kind == ambiguous_part(close_token.kind)
            {
                input.next()?;
                input.pop_format(close_token.kind);

                let mut cached_token = close_token;
                //TODO: set correct positions for cached token
                cached_token.kind = counterpart(close_token.kind);

                input.cache_token(cached_token);
                return Some(super::to_formatting(
                    close_token.kind,
                    inner,
                    None, // check for optional attributes here
                    open_token.start,
                    close_token.end,
                ));
            } else if open_token.kind == ambiguous_part(close_token.kind) {
                // no cache, because "split" is on open token
                input.next()?;
                input.pop_format(close_token.kind);
                outer.push(super::to_formatting(
                    close_token.kind,
                    inner,
                    None, // check for optional attributes here
                    open_token.start,
                    close_token.end,
                ));

                open_token.kind = counterpart(close_token.kind);
                open_token
            } else {
                // Close open format, but do not consume close, because close is not compatible with open one
                // This means that some outer format closed.
                // => end of this format is at start of the closing token for the outer format
                return Some(close_format(input, open_token, inner, close_token.start));
            }
        }
        None => {
            // close open format only and return
            // This is ok, because if ambiguous would have been split, peek() would have returned the partial closing token

            //TODO: Update end position
            return Some(close_format(input, open_token, inner, open_token.start));
        }
    };

    outer.append(&mut inline_parser::InlineParser::default().parse(input));

    if let Some(mut close_token) = input.peek() {
        // open token was updated to either italic or bold from italicbold
        if compatible(updated_open.kind, close_token.kind) {
            input.next()?;

            if is_ambiguous(close_token.kind) {
                // ambiguous token gets split, because only part of it is used to close this open format
                // TODO: update positions
                close_token.kind = counterpart(updated_open.kind);
                input.cache_token(close_token);
            } else {
                debug_assert!(
                    updated_open.kind == close_token.kind,
                    "Closing '{:?}' did not match updated open '{:?}'.",
                    close_token.kind,
                    updated_open.kind
                );
                // check for optional attributes here
            }

            input.pop_format(updated_open.kind);
            return Some(super::to_formatting(
                updated_open.kind,
                outer,
                None,
                updated_open.start,
                close_token.end,
            ));
        }
    }

    //TODO: update end position
    input.pop_format(updated_open.kind);
    Some(super::to_formatting(
        updated_open.kind,
        outer,
        None,
        updated_open.start,
        updated_open.end,
    ))
}

fn close_format(
    input: &mut InlineTokenIterator<'_>,
    open_token: InlineToken<'_>,
    inner: Vec<Inline>,
    end: Position,
) -> Inline {
    if is_ambiguous(open_token.kind) {
        input.pop_format(main_part(open_token.kind));
        input.pop_format(sub_part(open_token.kind));

        //TODO: set correct positions
        super::to_formatting(
            main_part(open_token.kind),
            vec![super::to_formatting(
                sub_part(open_token.kind),
                inner,
                None,
                open_token.start,
                end,
            )],
            None,
            open_token.start,
            end,
        )
    } else {
        input.pop_format(open_token.kind);
        super::to_formatting(open_token.kind, inner, None, open_token.start, end)
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

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::token::iterator::TokenIterator;

    use crate::element::{
        formatting::{Subscript, Underline},
        plain::Plain,
    };

    use super::*;

    #[test]
    fn parse_underline_subscript() {
        let symbols = unimarkup_commons::scanner::scan_str("___underline__subscript_");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Subscript {
                inner: vec![
                    Underline {
                        inner: vec![Plain {
                            content: "underline".to_string(),
                        }
                        .into()]
                    }
                    .into(),
                    Plain {
                        content: "subscript".to_string(),
                    }
                    .into()
                ],
            }
            .into(),
            "Subscript + underline not correctly parsed."
        )
    }

    #[test]
    fn parse_underline_subscript_ambiguous_close() {
        let symbols = unimarkup_commons::scanner::scan_str("__underline_subscript___");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Underline {
                inner: vec![
                    Plain {
                        content: "underline".to_string(),
                    }
                    .into(),
                    Subscript {
                        inner: vec![Plain {
                            content: "subscript".to_string(),
                        }
                        .into()]
                    }
                    .into(),
                ],
            }
            .into(),
            "Subscript + underline not correctly parsed."
        )
    }
}
