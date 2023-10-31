use std::rc::Rc;

use unimarkup_commons::{
    lexer::token::{implicit::iterator::TokenIteratorImplicitExt, iterator::EndMatcher},
    parsing::InlineContext,
};

use crate::{
    element::Inline,
    inline_parser,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind},
};

macro_rules! scoped_parser {
    ($fn_name:ident, $kind:ident) => {
        pub(crate) fn $fn_name(
            input: &mut InlineTokenIterator,
            context: &mut InlineContext,
        ) -> Option<Inline> {
            let open_token = input.next()?;

            // No need to check for correct opening format, because parser is only assigned for valid opening tokens.
            if input.peek_kind()?.is_space() {
                return None;
            }

            let mut scoped_iter: InlineTokenIterator<'_> = input
                .nest_with_scope(Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                    !matcher.prev_is_space()
                        && matcher.consumed_matches(&[InlineTokenKind::$kind.into()])
                })))
                .into();

            // ignore implicits, because only escapes and logic elements are allowed in following inline verbatim
            let prev_implicits_allowed = scoped_iter.implicits_allowed();
            scoped_iter.ignore_implicits();

            let prev_context_flags = context.flags;
            context.flags.keep_whitespaces = true;
            context.flags.logic_only = true;

            let inner = inline_parser::parse(&mut scoped_iter, context);

            context.flags = prev_context_flags;
            if prev_implicits_allowed {
                scoped_iter.allow_implicits();
            }

            let end_reached = scoped_iter.end_reached();
            scoped_iter.update(input);

            let prev_token = input.prev_token().expect(
                "Previous token must exist, because peek above would else have returned None.",
            );
            dbg!(&prev_token);
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

            Some(super::to_formatting(
                open_token.kind,
                inner,
                attributes,
                open_token.start,
                end,
                implicit_end,
            ))
        }
    };
}

scoped_parser!(parse_verbatim, Verbatim);
scoped_parser!(parse_math, Math);
