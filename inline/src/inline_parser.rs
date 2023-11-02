//! Inline parser

use unimarkup_commons::{lexer::token::iterator::TokenIterator, parsing::InlineContext};

use crate::{
    element::Inline,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind},
};

/// Parser function type for inline element parsing.
pub(crate) type InlineParserFn =
    for<'s, 'i> fn(&mut InlineTokenIterator<'s, 'i>, &mut InlineContext) -> Option<Inline>;

/// Creates inline elements using the given token iterator.
pub fn parse_inlines(
    token_iter: TokenIterator<'_, '_>,
    context: &mut InlineContext,
) -> Vec<Inline> {
    parse(
        &mut InlineTokenIterator::from(TokenIterator::with_scoped_root(token_iter)),
        context,
    )
}

pub(crate) fn parse(input: &mut InlineTokenIterator, context: &mut InlineContext) -> Vec<Inline> {
    let mut inlines = Vec::default();
    let mut format_closes = false;

    #[cfg(debug_assertions)]
    let mut curr_len = input.max_len();

    input.reset_peek();

    'outer: while let Some(kind) = input.peek_kind() {
        if kind == InlineTokenKind::Eoi {
            break 'outer;
        }

        if (!context.flags.logic_only && kind.is_scoped_format_keyword())
            || kind.is_open_parenthesis()
        {
            if let Some(parser_fn) = get_scoped_parser(kind, context.flags.logic_only) {
                if let Some(res_inline) = parser_fn(input, context) {
                    inlines.push(res_inline);
                    continue 'outer;
                }
            }
        } else if !context.flags.logic_only && kind.is_format_keyword() {
            // An open format closes => unwrap to closing format element
            // closing token is not consumed here => the element parser needs this info
            if input.format_closes(kind) {
                format_closes = true;
                break 'outer;
            } else if !input.format_is_open(kind) {
                if let Some(parser_fn) = get_format_parser(kind) {
                    if let Some(res_inline) = parser_fn(input, context) {
                        inlines.push(res_inline);
                        continue 'outer;
                    }
                }
            }
        }

        crate::element::base::parse_base(input, context, &mut inlines);

        #[cfg(debug_assertions)]
        {
            assert!(
                input.max_len() < curr_len,
                "Parser consumed no symbol in iteration."
            );
            curr_len = input.max_len();
        }
    }

    if !format_closes {
        // To consume tokens in end matching, but do not consume closing formatting tokens
        let _ = input.next();
    }

    inlines
}

fn get_format_parser(kind: InlineTokenKind) -> Option<InlineParserFn> {
    match kind {
        InlineTokenKind::Bold
        | InlineTokenKind::Italic
        | InlineTokenKind::BoldItalic
        | InlineTokenKind::Underline
        | InlineTokenKind::Subscript
        | InlineTokenKind::UnderlineSubscript => Some(crate::element::formatting::ambiguous::parse),
        InlineTokenKind::Strikethrough
        | InlineTokenKind::Superscript
        | InlineTokenKind::Highlight
        | InlineTokenKind::Overline
        | InlineTokenKind::Quote => Some(crate::element::formatting::parse_distinct_format),
        _ => None,
    }
}

fn get_scoped_parser(kind: InlineTokenKind, logic_only: bool) -> Option<InlineParserFn> {
    match kind {
        InlineTokenKind::Verbatim if !logic_only => {
            Some(crate::element::formatting::scoped::parse_verbatim)
        }
        InlineTokenKind::Math if !logic_only => {
            Some(crate::element::formatting::scoped::parse_math)
        }
        InlineTokenKind::OpenBracket if !logic_only => Some(crate::element::textbox::parse),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::{lexer::token::iterator::TokenIterator, parsing::InlineContext};

    use crate::tokenize::iterator::InlineTokenIterator;

    #[test]
    fn dummy_for_debugging() {
        let tokens = unimarkup_commons::lexer::token::lex_str("`a`");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*tokens));

        let inlines = super::parse(&mut token_iter, &mut InlineContext::default());

        // dbg!(&inlines);

        assert!(!inlines.is_empty(), "No inlines created.");
        assert_eq!(token_iter.next(), None, "Iterator not fully consumed.");
    }
}
