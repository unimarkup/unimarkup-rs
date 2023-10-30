use std::rc::Rc;

use unimarkup_commons::{
    lexer::token::{implicit::iterator::TokenIteratorImplicitExt, iterator::EndMatcher},
    parsing::Context,
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
            context: &mut Context,
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
            // ignore implicits, because only escapes and macros are allowed in following inlines
            scoped_iter.ignore_implicits();
            context.keep_spaces = true;
            context.macros_only = true;
            let inner = inline_parser::parse(&mut scoped_iter, context);
            context.keep_spaces = false;
            context.macros_only = false;
            scoped_iter.allow_implicits();

            let prev_token = scoped_iter.prev_token().expect(
                "Previous token must exist, because peek above would else have returned None.",
            );

            let (attributes, end, implicit_end) = if scoped_iter.end_reached() {
                //TODO: Check for optional attributes here
                (None, prev_token.end, false)
            } else {
                (None, prev_token.start, true)
            };

            scoped_iter.update(input);

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
