use std::rc::Rc;

use unimarkup_commons::{
    parser::{GroupParser, Parser},
    scanner::{EndMatcher, Itertools, SymbolKind},
};

use crate::{
    element::{Inline, InlineElement, InlineError},
    inline_parser,
    new_parser::InlineParser,
    tokenize::{
        iterator::InlineTokenIterator,
        token::{InlineToken, InlineTokenKind},
    },
};

use super::{Bold, Italic, BOLD_ITALIC_KEYWORD_LIMIT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoldItalic {
    Bold(Bold),
    Italic(Italic),
}

pub fn parse(input: &mut InlineTokenIterator) -> Option<Inline> {
    let mut open_token = input.next()?;

    if input.peek_kind()?.is_space() {
        // Split ambiguous in case of leading space. Bold wins
        if open_token.kind == InlineTokenKind::ItalicBold {
            let mut cached = open_token;
            cached.kind = InlineTokenKind::Italic;

            //TODO: update spans

            input.cache_token(cached);
            open_token.kind = InlineTokenKind::Bold;
        } else {
            return None;
        }
    }

    match open_token.kind {
        InlineTokenKind::ItalicBold => {
            input.push_format(InlineTokenKind::Italic);
            input.push_format(InlineTokenKind::Bold);
        }
        InlineTokenKind::Italic | InlineTokenKind::Bold => {
            input.push_format(open_token.kind);
        }
        _ => {
            return None;
        }
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
        Some(mut close_token) => {
            if open_token.kind == InlineTokenKind::Bold
                && close_token.kind == InlineTokenKind::Italic
            {
                // open = bold, close = italic => italic was opened in other parser => close bold, but do not consume close and no second part
                input.pop_format(InlineTokenKind::Bold);
                return Some(Bold { inner }.into());
            } else if open_token.kind == InlineTokenKind::Italic
                && close_token.kind == InlineTokenKind::Bold
            {
                // open = italic, close = bold => bold was opened in other parser => close italic, but do not consume close and no second part
                input.pop_format(InlineTokenKind::Italic);
                return Some(Italic { inner }.into());
            } else if open_token.kind == InlineTokenKind::Italic
                && close_token.kind == InlineTokenKind::Italic
            {
                // open & close = italic => close italic and consume close and no second part
                input.next()?;
                input.pop_format(InlineTokenKind::Italic);
                return Some(Italic { inner }.into());
            } else if open_token.kind == InlineTokenKind::Bold
                && close_token.kind == InlineTokenKind::Bold
            {
                // open & close = bold => close bold and consume close and no second part
                input.next()?;
                input.pop_format(InlineTokenKind::Bold);
                return Some(Bold { inner }.into());
            } else if open_token.kind == InlineTokenKind::ItalicBold
                && close_token.kind == InlineTokenKind::ItalicBold
            {
                // open & close = italicbold => close italicbold and consume close and no second part
                input.next()?;
                input.pop_format(InlineTokenKind::ItalicBold);
                return Some(
                    Bold {
                        inner: vec![Italic { inner }.into()],
                    }
                    .into(),
                );
            } else if open_token.kind == InlineTokenKind::Bold
                && close_token.kind == InlineTokenKind::ItalicBold
            {
                // open = bold, close = italicbold => close bold, consume close, cache italic and no second part, because italic is handled by other parser
                input.next()?;
                input.pop_format(InlineTokenKind::Bold);

                close_token.kind = InlineTokenKind::Italic;
                input.cache_token(close_token);
                return Some(Bold { inner }.into());
            } else if open_token.kind == InlineTokenKind::Italic
                && close_token.kind == InlineTokenKind::ItalicBold
            {
                // open = italic, close = italicbold => close italic, consume close, cache bold and no second part, because bold is handled by other parser
                input.next()?;
                input.pop_format(InlineTokenKind::Italic);

                close_token.kind = InlineTokenKind::Bold;
                input.cache_token(close_token);
                return Some(Italic { inner }.into());
            } else if open_token.kind == InlineTokenKind::ItalicBold
                && close_token.kind == InlineTokenKind::Bold
            {
                // open = italicbold, close = bold => close bold, consume close and parse second part (split span of open)
                // no cache, because "split" is on open token
                input.next()?;
                input.pop_format(InlineTokenKind::Bold);
                outer.push(Bold { inner }.into());

                open_token.kind = InlineTokenKind::Italic;
                open_token
            } else if open_token.kind == InlineTokenKind::ItalicBold
                && close_token.kind == InlineTokenKind::Italic
            {
                // open = italicbold, close = italic => close italic, consume close and parse second part
                // no cache, because "split" is on open token
                input.next()?;
                input.pop_format(InlineTokenKind::Italic);
                outer.push(Italic { inner }.into());

                open_token.kind = InlineTokenKind::Bold;
                //TODO: update spans & prev_token
                open_token
            } else {
                // closing token is not compatible with bold or italic => other outer format closed
                // close open format, but do not consume close
                if open_token.kind == InlineTokenKind::ItalicBold {
                    input.pop_format(InlineTokenKind::Italic);
                    input.pop_format(InlineTokenKind::Bold);

                    return Some(
                        Bold {
                            inner: vec![Italic { inner }.into()],
                        }
                        .into(),
                    );
                } else if open_token.kind == InlineTokenKind::Italic {
                    input.pop_format(open_token.kind);
                    return Some(Italic { inner }.into());
                } else {
                    input.pop_format(open_token.kind);
                    return Some(Bold { inner }.into());
                }
            }
        }
        None => {
            // close open format only and return
            // This is ok, because if ambiguous would have been split, peek() would have returned the partial closing token
            if open_token.kind == InlineTokenKind::ItalicBold {
                input.pop_format(InlineTokenKind::Italic);
                input.pop_format(InlineTokenKind::Bold);

                return Some(
                    Bold {
                        inner: vec![Italic { inner }.into()],
                    }
                    .into(),
                );
            } else if open_token.kind == InlineTokenKind::Italic {
                input.pop_format(open_token.kind);
                return Some(Italic { inner }.into());
            } else {
                input.pop_format(open_token.kind);
                return Some(Bold { inner }.into());
            }
        }
    };

    outer.append(&mut inline_parser::InlineParser::default().parse(input));

    match input.peek() {
        Some(mut close_token) => {
            // open token was updated to either italic or bold from italicbold
            if close_token.kind == InlineTokenKind::Italic {
                // updated open = italic, close = italic => close italic, consume close
                debug_assert!(
                    updated_open.kind == InlineTokenKind::Italic,
                    "Closing italic did not match updated open token."
                );

                input.next()?;
                input.pop_format(close_token.kind);
                Some(Italic { inner: outer }.into())
            } else if close_token.kind == InlineTokenKind::Bold {
                // updated open = bold, close = bold => close bold, consume close
                debug_assert!(
                    updated_open.kind == InlineTokenKind::Bold,
                    "Closing bold did not match updated open token."
                );

                input.next()?;
                input.pop_format(close_token.kind);
                Some(Bold { inner: outer }.into())
            } else if updated_open.kind == InlineTokenKind::Italic
                && close_token.kind == InlineTokenKind::ItalicBold
            {
                // updated open = italic, close = italicbold => close italic, consume close, cache bold
                input.next()?;
                input.pop_format(InlineTokenKind::Italic);

                close_token.kind = InlineTokenKind::Bold;
                input.cache_token(close_token);
                Some(Italic { inner: outer }.into())
            } else if updated_open.kind == InlineTokenKind::Bold
                && close_token.kind == InlineTokenKind::ItalicBold
            {
                // updated open = bold, close = italicbold => close bold, consume close, cache italic
                input.next()?;
                input.pop_format(InlineTokenKind::Bold);

                close_token.kind = InlineTokenKind::Italic;
                input.cache_token(close_token);
                Some(Bold { inner: outer }.into())
            } else {
                // close neither italic, bold, italicbold => close format that was not closed in intermediate, but do not consume close => outer format closed
                if updated_open.kind == InlineTokenKind::Italic {
                    input.pop_format(updated_open.kind);
                    Some(Italic { inner: outer }.into())
                } else {
                    input.pop_format(updated_open.kind);
                    Some(Bold { inner: outer }.into())
                }
            }
        }
        None => {
            // close format that was not closed above
            if updated_open.kind == InlineTokenKind::Italic {
                input.pop_format(updated_open.kind);
                Some(Italic { inner: outer }.into())
            } else {
                input.pop_format(updated_open.kind);
                Some(Bold { inner: outer }.into())
            }
        }
    }
}

impl InlineElement for BoldItalic {}

impl Parser<Inline> for BoldItalic {
    fn parse(input: &mut unimarkup_commons::scanner::SymbolIterator) -> Option<Inline> {
        let first_symbol = input.next()?;
        let second_symbol = input.next()?;
        let third_symbol = input.next()?;
        let fourth_symbol = input.peek()?;

        if first_symbol.kind != SymbolKind::Star
            || second_symbol.kind != SymbolKind::Star
            || third_symbol.kind != SymbolKind::Star
            || fourth_symbol.kind == SymbolKind::Star
            || fourth_symbol.kind.is_space()
        {
            return None;
        }

        let mut inner_iter = input.nest_scoped(
            None,
            Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                !matcher.prev_is_space()
                // Contiguous keywords are consumed in inline parser
                && !matcher.matches(BOLD_ITALIC_KEYWORD_LIMIT)
                && matcher.matches(&[SymbolKind::Star])
            })),
        );

        let inline_parser = InlineParser::default();
        let inner = inline_parser.parse(&mut inner_iter);
        let inner_end = inner_iter.end_reached();

        inner_iter.update(input);

        let mut star_iter = input.nest_scoped(None, None);
        let star_cnt = star_iter
            .peeking_take_while(|s| s.kind == SymbolKind::Star)
            .count();

        star_iter.by_ref().dropping(star_cnt);
        star_iter.update(input);

        if !inner_end || star_cnt > 2 || star_cnt == 0 {
            return Some(
                Bold {
                    inner: vec![Italic { inner }.into()],
                }
                .into(),
            );
        }

        let mut outer_iter = if star_cnt == 1 {
            // Italic closed => outer is Bold
            input.nest_scoped(
                None,
                Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                    !matcher.prev_is_space()
                        // Contiguous keywords are consumed in inline parser
                        && !matcher.matches(BOLD_ITALIC_KEYWORD_LIMIT)
                        && matcher.consumed_matches(&[SymbolKind::Star, SymbolKind::Star])
                })),
            )
        } else {
            // Bold closed => outer is Italic
            input.nest_scoped(
                None,
                Some(Rc::new(|matcher: &mut dyn EndMatcher| {
                    !matcher.prev_is_space()
                        && !matcher.matches(&[SymbolKind::Star, SymbolKind::Star])
                        && matcher.consumed_matches(&[SymbolKind::Star])
                })),
            )
        };

        let inline_parser = InlineParser::default();
        let outer = inline_parser.parse(&mut outer_iter);

        outer_iter.update(input);

        if star_cnt == 1 {
            let mut content = vec![Italic { inner }.into()];
            content.extend(outer);
            Some(Bold { inner: content }.into())
        } else {
            let mut content = vec![Bold { inner }.into()];
            content.extend(outer);
            Some(Italic { inner: content }.into())
        }
    }
}

impl From<BoldItalic> for Inline {
    fn from(value: BoldItalic) -> Self {
        match value {
            BoldItalic::Bold(bold) => Inline::Bold(bold),
            BoldItalic::Italic(italic) => Inline::Italic(italic),
        }
    }
}

impl TryFrom<Inline> for BoldItalic {
    type Error = InlineError;

    fn try_from(value: Inline) -> Result<Self, Self::Error> {
        match value {
            Inline::Bold(bold) => Ok(BoldItalic::Bold(bold)),
            Inline::Italic(italic) => Ok(BoldItalic::Italic(italic)),
            _ => Err(InlineError::ConversionMismatch),
        }
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::{token::iterator::TokenIterator, SymbolIterator};

    use crate::element::{plain::Plain, spaces::Whitespace};

    use super::*;

    #[test]
    fn parse_bold_italic() {
        let symbols = unimarkup_commons::scanner::scan_str("***bold**italic*");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Italic {
                inner: vec![
                    Bold {
                        inner: vec![Plain {
                            content: "bold".to_string(),
                        }
                        .into()]
                    }
                    .into(),
                    Plain {
                        content: "italic".to_string(),
                    }
                    .into()
                ],
            }
            .into(),
            "Bold + italic not correctly parsed."
        )
    }

    #[test]
    fn parse_italic_bold() {
        let symbols = unimarkup_commons::scanner::scan_str("***italic*bold**");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Bold {
                inner: vec![
                    Italic {
                        inner: vec![Plain {
                            content: "italic".to_string(),
                        }
                        .into()]
                    }
                    .into(),
                    Plain {
                        content: "bold".to_string(),
                    }
                    .into()
                ],
            }
            .into(),
            "Bold + italic not correctly parsed."
        )
    }

    #[test]
    fn parse_new_bold_italic_closing_bolditalic() {
        let symbols = unimarkup_commons::scanner::scan_str("**bold*italic***");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            inline,
            Bold {
                inner: vec![
                    Plain {
                        content: "bold".to_string(),
                    }
                    .into(),
                    Italic {
                        inner: vec![Plain {
                            content: "italic".to_string(),
                        }
                        .into()]
                    }
                    .into(),
                ],
            }
            .into(),
            "Bold + italic not correctly parsed."
        )
    }

    #[test]
    fn parse_bold_before_italic() {
        let symbols = unimarkup_commons::scanner::scan_str("**bold***italic*");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let bold = parse(&mut token_iter).unwrap();
        let italic = parse(&mut token_iter).unwrap();

        assert_eq!(
            bold,
            Bold {
                inner: vec![Plain {
                    content: "bold".to_string(),
                }
                .into(),],
            }
            .into(),
            "Bold not correctly parsed."
        );
        assert_eq!(
            italic,
            Italic {
                inner: vec![Plain {
                    content: "italic".to_string(),
                }
                .into(),],
            }
            .into(),
            "Italic not correctly parsed."
        )
    }

    #[test]
    fn parse_italic_bold_with_contiguous_stars() {
        let symbols = unimarkup_commons::scanner::scan_str("***italic*a****a**");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = parse(&mut token_iter).unwrap();

        assert_eq!(
            Bold::try_from(inline).unwrap(),
            Bold {
                inner: vec![
                    Italic {
                        inner: vec![Plain {
                            content: "italic".to_string(),
                        }
                        .into()]
                    }
                    .into(),
                    Plain {
                        content: "a****a".to_string(),
                    }
                    .into()
                ],
            },
            "Italic + bold not correctly parsed."
        );

        assert_eq!(
            token_iter.next().unwrap().kind,
            InlineTokenKind::EOI,
            "Iterator returned symbols after EOI"
        );
    }

    // TODO: fix offset in ambiguous split to get correct plain conversion
    // #[test]
    // fn parse_bold_before_invalid_close_italicbold() {
    //     let symbols = unimarkup_commons::scanner::scan_str("**bold ***");
    //     let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

    //     let bold = parse(&mut token_iter).unwrap();

    //     assert_eq!(
    //         bold,
    //         Bold {
    //             inner: vec![
    //                 Plain {
    //                     content: "bold".to_string(),
    //                 }
    //                 .into(),
    //                 Whitespace {}.into(),
    //                 Plain {
    //                     content: "*".to_string(),
    //                 }
    //                 .into()
    //             ],
    //         }
    //         .into(),
    //         "Bold not correctly parsed."
    //     );
    // }
}
