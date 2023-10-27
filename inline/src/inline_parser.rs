//! Inline parser

use unimarkup_commons::scanner::{token::iterator::TokenIterator, SymbolIterator};

use crate::{
    element::Inline,
    tokenize::{iterator::InlineTokenIterator, token::InlineTokenKind},
};

/// Parser function scoped elements must provide
pub type InlineParserFn = for<'i> fn(&mut InlineTokenIterator<'i>) -> Option<Inline>;

/// Main parser for Unimarkup inline elements.
#[derive(Clone)]
pub struct InlineParser {
    handle_formats: bool,
    //TODO: use hashmap with InlineTokenKind for parser fns, because every kind has at most one parser
    scoped_parsers: Vec<InlineParserFn>,
}

impl Default for InlineParser {
    fn default() -> Self {
        Self {
            handle_formats: true,
            scoped_parsers: Vec::with_capacity(2),
        }
    }
}

impl InlineParser {
    /// Creates inline elements from the given symbol iterator.
    pub fn parse_inlines(token_iter: TokenIterator) -> Vec<Inline> {
        InlineParser::default().parse(&mut InlineTokenIterator::from(
            TokenIterator::with_scope_root(token_iter),
        ))
    }

    pub(crate) fn parse(&self, input: &mut InlineTokenIterator) -> Vec<Inline> {
        let mut inlines = Vec::default();
        let mut format_closes = false;

        #[cfg(debug_assertions)]
        let mut curr_len = input.max_len();

        input.reset_peek();

        'outer: while let Some(kind) = input.peek_kind() {
            // TODO: handle implicit substitutions of last if kind is space and last inline is plain

            if kind == InlineTokenKind::EOI {
                break 'outer;
            }

            if kind.is_scoped_format_keyword() || kind.is_open_parenthesis() {
                for parser_fn in &self.scoped_parsers {
                    let mut iter = input.clone();
                    if let Some(res_inline) = parser_fn(&mut iter) {
                        inlines.push(res_inline);
                        *input = iter;
                        continue 'outer;
                    }
                }
            } else if self.handle_formats && kind.is_format_keyword() {
                // An open format closes => unwrap to closing format element
                // closing token is not consumed here => the element parser needs this info
                if input.format_closes(kind) {
                    format_closes = true;
                    break 'outer;
                } else if !input.format_is_open(kind) {
                    if let Some(parser_fn) = get_format_parser(kind) {
                        let mut iter = input.clone();
                        if let Some(res_inline) = parser_fn(&mut iter) {
                            inlines.push(res_inline);
                            *input = iter;
                            continue 'outer;
                        }
                    }
                }
            }

            let mut next = input.next().expect("Peeked symbol before.");

            if kind.is_keyword() {
                // Ambiguous token may be split to get possible valid partial token
                input.ambiguous_split(&mut next);

                // If keyword was not handled above => convert token to plain
                next.kind = InlineTokenKind::Plain;
                input.set_prev_token(next); // update prev token, because next changed afterwards
            }

            match inlines.last_mut() {
                Some(last) => match last {
                    Inline::Plain(plain) if next.kind == InlineTokenKind::Plain => {
                        plain.content.push_str(next.as_str());
                    }
                    _ => inlines.push(next.into()),
                },
                None => inlines.push(next.into()),
            }

            // #[cfg(debug_assertions)]
            // {
            //     assert!(
            //         input.max_len() < curr_len,
            //         "Parser consumed no symbol in iteration."
            //     );
            //     curr_len = input.max_len();
            // }
        }

        // TODO: check for implicit substitutions if last is plain...

        if !format_closes {
            // To consume tokens in end matching, but do not consume closing formatting tokens
            let _ = input.next();
        }

        inlines
    }
}

fn get_format_parser(kind: InlineTokenKind) -> Option<InlineParserFn> {
    match kind {
        InlineTokenKind::Bold | InlineTokenKind::Italic | InlineTokenKind::ItalicBold => {
            Some(crate::element::formatting::bold_italic::parse)
        }
        InlineTokenKind::Strikethrough => Some(crate::element::formatting::strikethrough::parse),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::token::iterator::TokenIterator;

    use crate::{
        element::{
            formatting::{bold_italic::Bold, strikethrough::Strikethrough},
            plain::Plain,
        },
        inline_parser::InlineParser,
        tokenize::iterator::InlineTokenIterator,
    };

    #[test]
    fn parse_strikethrough_in_unclosed_bold() {
        let symbols = unimarkup_commons::scanner::scan_str("**~~strikethrough~~");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inlines = InlineParser::default().parse(&mut token_iter);

        assert_eq!(
            inlines.len(),
            1,
            "Parser did not return one inline element."
        );

        assert_eq!(
            Bold::try_from(inlines[0].clone()).unwrap(),
            Bold {
                inner: vec![Strikethrough {
                    inner: vec![Plain {
                        content: "strikethrough".to_string(),
                    }
                    .into()],
                }
                .into()],
            },
            "Strikethrough not correctly parsed."
        );

        assert_eq!(token_iter.next(), None, "Iterator not fully consumed.");
    }
}
