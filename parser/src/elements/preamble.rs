//! [`preamble`](crate::frontend::preamble) is the module which implements parsing of the preamble and merge the config of the preamble with the CLI arguments.

use std::rc::Rc;

use unimarkup_commons::{
    config::preamble::Preamble,
    lexer::token::{
        iterator::{EndMatcher, PeekingNext},
        Token, TokenKind,
    },
};

use crate::BlockParser;

pub(crate) fn parse_preamble<'s, 'i>(
    mut parser: BlockParser<'s, 'i>,
) -> (BlockParser<'s, 'i>, Option<Preamble>) {
    match parser.iter.peeking_next(|_| true) {
        Some(token) => {
            if let TokenKind::Plus(len) = token.kind {
                if len < 3
                    || parser
                        .iter
                        .peeking_next(|t| t.kind == TokenKind::Newline)
                        .is_none()
                {
                    return (parser, None);
                }
            } else {
                return (parser, None);
            }
        }
        None => return (parser, None),
    };

    let preamble_start = parser
        .iter
        .next()
        .expect("Ensured next is Some with peek above.");
    parser.iter.next(); // Consume newline

    let preamble_kind = preamble_start.kind;

    let mut preamble_parser = parser.nest(
        None,
        Some(Rc::new(move |matcher: &mut dyn EndMatcher| {
            matcher.consumed_matches(&[preamble_kind, TokenKind::Blankline])
        })),
    );

    let preamble_tokens = preamble_parser.iter.take_to_end();
    parser = preamble_parser.into_inner();
    let content = Token::flatten_ref(&preamble_tokens);

    match content {
        Some(preamble_content) => match serde_yaml::from_str(preamble_content) {
            Ok(preamble) => (parser, Some(preamble)),
            Err(_) => (parser, None),
        },
        None => (parser, None),
    }
}
