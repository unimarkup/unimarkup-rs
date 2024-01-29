//! Contains the parser for scoped formats like [`Math`](super::Math).

use std::rc::Rc;

use unimarkup_commons::lexer::token::iterator::{EndMatcher, PeekingNext};

use crate::{element::Inline, parser::InlineParser, tokenize::kind::InlineTokenKind};

macro_rules! scoped_parser {
    ($fn_name:ident, $kind:ident) => {
        pub(crate) fn $fn_name<'slice, 'input>(
            mut parser: InlineParser<'slice, 'input>,
        ) -> (InlineParser<'slice, 'input>, Option<Inline>) {
            let Some(open_token) = parser.iter.peeking_next(|_| true) else {
                return (parser, None);
            };

            // No need to check for correct opening format, because parser is only assigned for valid opening tokens.
            if parser.iter.peek_kind().map_or(true, |t| t.is_space()) {
                return (parser, None);
            }

            parser.iter.next(); // consume open token => now it will lead to Some(inline)

            // ignore implicits, because only escapes and logic elements are allowed in following inline verbatim
            let prev_context_flags = parser.context.flags;

            let (mut scoped_parser, outer_open_formats) =
                parser.nest_scoped(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                    !matcher.prev_is_space()
                        && matcher.consumed_matches(&[InlineTokenKind::$kind.into()])
                })));

            scoped_parser.context.flags.allow_implicits = false;
            scoped_parser.context.flags.keep_whitespaces = true;
            scoped_parser.context.flags.logic_only = true;

            let (updated_parser, inner) = InlineParser::parse(scoped_parser);
            scoped_parser = updated_parser;

            let end_reached = scoped_parser.iter.end_reached();
            parser = scoped_parser.unfold(outer_open_formats);
            parser.context.flags = prev_context_flags;

            let prev_token = parser.iter.prev_token().expect(
                "Previous token must exist, because peek above would else have returned None.",
            );

            let (attributes, end, implicit_end) = if end_reached {
                //TODO: Check for optional attributes here
                (None, prev_token.end, false)
            } else {
                (
                    None,
                    $crate::element::helper::implicit_end_using_prev(&prev_token),
                    true,
                )
            };

            (
                parser,
                Some(super::to_formatting(
                    open_token.kind,
                    inner,
                    attributes,
                    open_token.start,
                    end,
                    implicit_end,
                )),
            )
        }
    };
}

scoped_parser!(parse_verbatim, Verbatim);
scoped_parser!(parse_math, Math);
