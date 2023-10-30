//! Inline parser

use unimarkup_commons::{lexer::token::iterator::TokenIterator, parsing::InlineContext};

use crate::{
    element::Inline,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind},
};

/// Parser function type for inline element parsing.
pub(crate) type InlineParserFn =
    for<'i> fn(&mut InlineTokenIterator<'i>, &mut InlineContext) -> Option<Inline>;

/// Creates inline elements using the given token iterator.
pub fn parse_inlines<'input>(
    token_iter: impl Into<TokenIterator<'input>>,
    context: &mut InlineContext,
) -> Vec<Inline> {
    parse(
        &mut InlineTokenIterator::from(TokenIterator::with_scoped_root(token_iter.into())),
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
                let mut iter = input.clone();
                let mut inner_context = context.clone();
                if let Some(res_inline) = parser_fn(&mut iter, &mut inner_context) {
                    inlines.push(res_inline);
                    *input = iter;
                    *context = inner_context;
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
                    let mut iter = input.clone();
                    let mut inner_context = context.clone();
                    if let Some(res_inline) = parser_fn(&mut iter, &mut inner_context) {
                        inlines.push(res_inline);
                        *input = iter;
                        *context = inner_context;
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
    fn parse_strikethrough_in_unclosed_bold() {
        let symbols = unimarkup_commons::lexer::scan_str("(tm)");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inlines = super::parse(&mut token_iter, &mut InlineContext::default());

        dbg!(&inlines);
        // assert_eq!(
        //     Bold::try_from(inlines[0].clone()).unwrap(),
        //     Bold {
        //         inner: vec![Strikethrough {
        //             inner: vec![Plain {
        //                 content: "strikethrough".to_string(),
        //             }
        //             .into()],
        //         }
        //         .into()],
        //     },
        //     "Strikethrough not correctly parsed."
        // );

        assert_eq!(token_iter.next(), None, "Iterator not fully consumed.");
    }
}
//     #[test]
//     fn parse_textbox_scoped_bold() {
//         let symbols = unimarkup_commons::scanner::scan_str("**outer[**inner]");
//         let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

//         let inlines = InlineParser::default().parse(&mut token_iter);

//         assert_eq!(
//             inlines[0],
//             Bold {
//                 inner: vec![
//                     Plain {
//                         content: "outer".to_string(),
//                     }
//                     .into(),
//                     TextBox {
//                         inner: vec![Bold {
//                             inner: vec![Plain {
//                                 content: "inner".to_string(),
//                             }
//                             .into()],
//                         }
//                         .into(),],
//                     }
//                     .into(),
//                 ]
//             }
//             .into(),
//             "Textbox with scoped Bold not correctly parsed."
//         );
//     }

//     #[test]
//     fn parse_ambiguous_between() {
//         let symbols = unimarkup_commons::scanner::scan_str("__underline___subscript_");
//         let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

//         let inlines = InlineParser::default().parse(&mut token_iter);

//         assert_eq!(
//             inlines.len(),
//             2,
//             "Parser did not return two inline elements."
//         );

//         assert_eq!(
//             inlines,
//             vec![
//                 Underline {
//                     inner: vec![Plain {
//                         content: "underline".to_string(),
//                     }
//                     .into()],
//                 }
//                 .into(),
//                 Subscript {
//                     inner: vec![Plain {
//                         content: "subscript".to_string(),
//                     }
//                     .into()],
//                 }
//                 .into()
//             ],
//             "Underline + subscript not correctly parsed."
//         );

//         assert_eq!(token_iter.next(), None, "Iterator not fully consumed.");
//     }
// }
