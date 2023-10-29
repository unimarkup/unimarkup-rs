use std::rc::Rc;

use unimarkup_commons::scanner::token::{
    implicit::iterator::TokenIteratorImplicitExt, iterator::EndMatcher,
};

use crate::{
    element::Inline,
    inline_parser,
    tokenize::{iterator::InlineTokenIterator, token::InlineTokenKind},
};

macro_rules! scoped_parser {
    ($fn_name:ident, $kind:ident) => {
        pub fn $fn_name(input: &mut InlineTokenIterator) -> Option<Inline> {
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
            let inner = inline_parser::parse_with_macros_only(&mut scoped_iter);
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
