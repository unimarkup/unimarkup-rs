//! Main inline parser.

use unimarkup_commons::{
    parser::{GroupParser, ParserFn, ParserGenerator},
    scanner::{Itertools, SymbolIterator, SymbolKind},
};

use crate::element::{
    formatting::{
        bold_italic::{Bold, BoldItalic, Italic, BOLD_ITALIC_KEYWORD_LIMIT},
        strikethrough::{Strikethrough, STRIKETHROUGH_KEYWORD_LIMIT},
        verbatim::{Verbatim, VERBATIM_KEYWORD_LIMIT},
    },
    plain::{EscapedPlain, Plain},
    spaces::{EscapedNewline, EscapedWhitespace, Newline, Whitespace},
    textbox::TextBox,
    Inline,
};

/// Main parser for Unimarkup inline elements.
#[derive(Clone)]
pub struct InlineParser {
    parsers: Vec<ParserFn<Inline>>,
    default_parser: ParserFn<Inline>,
}

impl InlineParser {
    /// Creates an [`InlineParser`] that only parses [`Plain`], [`Whitespace`], [`Newline`], and their escaped variants.
    /// Individual elements may be added for parsing using [`GroupParser::register_parser`].
    pub fn base() -> Self {
        Self {
            parsers: Vec::new(),
            default_parser: Plain::generate_parser(),
        }
    }
}

impl Default for InlineParser {
    fn default() -> Self {
        let mut parser = Self {
            parsers: Vec::with_capacity(6),
            default_parser: Plain::generate_parser(),
        };

        parser.register_parser(TextBox::generate_parser());
        parser.register_parser(Verbatim::generate_parser());
        parser.register_parser(Italic::generate_parser());
        parser.register_parser(Bold::generate_parser());
        parser.register_parser(BoldItalic::generate_parser());
        parser.register_parser(Strikethrough::generate_parser());

        parser
    }
}

impl GroupParser<Inline> for InlineParser {
    fn register_parser(&mut self, parser: ParserFn<Inline>) {
        self.parsers.push(parser);
    }

    fn parse(&self, input: &mut SymbolIterator) -> Vec<Inline> {
        let mut inlines = Vec::default();

        #[cfg(debug_assertions)]
        let mut curr_len = input.max_len();

        input.reset_peek();

        'outer: while let Some(kind) = input.peek_kind() {
            let inline = match kind {
                // stop parsing when end of input is reached
                SymbolKind::EOI => break 'outer,

                SymbolKind::Whitespace => {
                    let _symbol = input
                        .next()
                        .expect("Peeked symbol not returned with `next()`.");
                    Inline::Whitespace(Whitespace {})
                }

                SymbolKind::Newline => {
                    let _symbol = input
                        .next()
                        .expect("Peeked symbol not returned with `next()`.");
                    Inline::Newline(Newline {})
                }

                // Escape one symbol
                SymbolKind::Backslash => {
                    let backslash = input
                        .next()
                        .expect("Peeked symbol not returned with `next()`.");

                    let mut escaped_scope_iter = input.nest_with_scope(None, None);
                    let escaped_inline = match escaped_scope_iter.next() {
                        Some(escaped_symbol) => match escaped_symbol.kind {
                            SymbolKind::Whitespace => EscapedWhitespace {
                                space: escaped_symbol.as_str().to_string(),
                            }
                            .into(),
                            SymbolKind::Newline => EscapedNewline {}.into(),
                            _ => EscapedPlain {
                                content: escaped_symbol.as_str().to_string(),
                            }
                            .into(),
                        },
                        None => Plain {
                            content: backslash.as_str().to_string(),
                        }
                        .into(),
                    };
                    escaped_scope_iter.update(input);

                    escaped_inline
                }

                // no parser will match, parse with default parser
                _ if kind.is_not_keyword() => (self.default_parser)(input)
                    .expect("Default parser failed parsing non-keyword."),

                // symbol is start of an inline element, some parser should match
                _ => {
                    match contiguous_keywords(input) {
                        Some(plain_keywords) => plain_keywords,
                        None => {
                            for parser_fn in &self.parsers {
                                let mut iter = input.clone();
                                if let Some(res_inline) = parser_fn(&mut iter) {
                                    inlines.push(res_inline);
                                    *input = iter;
                                    continue 'outer; // start from first parser on next input
                                }
                            }

                            // no registered parser matched -> use default parser
                            (self.default_parser)(input).expect(
                                "Default parser failed parsing content no other parser matched.",
                            )
                        }
                    }
                }
            };

            match inlines.last_mut() {
                Some(last) => match last {
                    Inline::Plain(_) if inline.is_plain() => {
                        last.merge_plain(
                            Plain::try_from(inline)
                                .expect("Plain check above ensures it is plain."),
                        )
                        .expect("Plain check above ensures it is plain.");
                    }
                    Inline::Whitespace(_) if inline.is_whitespace() => {
                        last.merge_whitespace(
                            Whitespace::try_from(inline)
                                .expect("Whitespace check above ensures it is plain."),
                        )
                        .expect("Whitespace check above ensures it is plain.");
                    }
                    // TODO: check for implicit substitutions if new inline is not plain anymore, but last is...
                    _ => inlines.push(inline),
                },
                // First inline => no need to check for implicit substitutions, because this is at best one plain grapheme so far
                None => inlines.push(inline),
            }

            #[cfg(debug_assertions)]
            {
                assert!(
                    input.max_len() < curr_len,
                    "Parser consumed no symbol in iteration."
                );
                curr_len = input.max_len();
            }
        }

        // TODO: check for implicit substitutions if last is plain...

        // To consume symbols in end matching
        let _ = input.next();

        inlines
    }
}

/// Parse contiguous keywords exceeding keyword limits as [`Plain`].
fn contiguous_keywords(iter: &mut SymbolIterator) -> Option<Inline> {
    // Scoping prevents parent iterators from matching keywords
    let mut scoped_iter = iter.nest_with_scope(None, None);

    let inline = consume_keywords(
        &mut scoped_iter,
        SymbolKind::Star,
        BOLD_ITALIC_KEYWORD_LIMIT.len(),
    )
    .or_else(|| {
        consume_keywords(
            &mut scoped_iter,
            SymbolKind::Tick,
            VERBATIM_KEYWORD_LIMIT.len(),
        )
    })
    .or_else(|| {
        consume_keywords(
            &mut scoped_iter,
            SymbolKind::Tilde,
            STRIKETHROUGH_KEYWORD_LIMIT.len(),
        )
    });

    if inline.is_some() {
        scoped_iter.update(iter);
    }

    inline
}

fn consume_keywords(
    iter: &mut SymbolIterator,
    kind: SymbolKind,
    lower_bound: usize,
) -> Option<Inline> {
    let keyword_len = iter.peeking_take_while(|s| s.kind == kind).count();
    iter.reset_peek();

    if keyword_len < lower_bound {
        None
    } else {
        let keywords = iter
            .by_ref()
            .take(keyword_len)
            .fold(String::new(), |mut combined, s| {
                combined.push_str(s.as_str());
                combined
            });

        Some(Plain { content: keywords }.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_escaped_whitespace() {
        let symbols = unimarkup_commons::scanner::scan_str("\\ ");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        assert_eq!(
            inlines.len(),
            1,
            "Parser returned more than one inline element."
        );
        assert_eq!(
            inlines.first().unwrap(),
            &Inline::EscapedWhitespace(EscapedWhitespace {
                space: " ".to_string()
            }),
            "Escaped whitespace not correctly parsed."
        );
        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_bold_italic_diff_start() {
        let symbols = unimarkup_commons::scanner::scan_str("*italic**bold***");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        // assert_eq!(
        //     inlines.len(),
        //     1,
        //     "Parser returned more than one inline element."
        // );
        assert_eq!(
            inlines.first().unwrap(),
            &Inline::Italic(Italic {
                inner: vec![
                    Plain {
                        content: "italic".to_string()
                    }
                    .into(),
                    Bold {
                        inner: vec![Plain {
                            content: "bold".to_string()
                        }
                        .into()]
                    }
                    .into()
                ]
            }),
            "Bold italic not correctly parsed."
        );
        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_escaped_star_before_italic() {
        let symbols = unimarkup_commons::scanner::scan_str("\\**italic*");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        assert_eq!(
            inlines.len(),
            2,
            "Parser did not return two inline elements."
        );
        assert_eq!(
            inlines.first().unwrap(),
            &Inline::EscapedPlain(EscapedPlain {
                content: "*".to_string()
            }),
            "Escaped star not correctly parsed."
        );
        assert_eq!(
            inlines.last().unwrap(),
            &Inline::Italic(Italic {
                inner: vec![Plain {
                    content: "italic".to_string()
                }
                .into()]
            }),
            "Italic after escaped star not correctly parsed."
        );
        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_escaped_star_in_italic() {
        let symbols = unimarkup_commons::scanner::scan_str("*ita\\*lic*");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        assert_eq!(
            inlines.len(),
            1,
            "Parser did not return two inline elements."
        );
        assert_eq!(
            inlines.first().unwrap(),
            &Inline::Italic(Italic {
                inner: vec![
                    Plain {
                        content: "ita".to_string()
                    }
                    .into(),
                    EscapedPlain {
                        content: "*".to_string()
                    }
                    .into(),
                    Plain {
                        content: "lic".to_string()
                    }
                    .into()
                ]
            }),
            "Italic with escaped star not correctly parsed."
        );
        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_bold_after_bold() {
        let symbols = unimarkup_commons::scanner::scan_str("**bold1** **bold2**");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        assert_eq!(
            inlines.len(),
            3,
            "Parser did not return three inline elements."
        );

        assert_eq!(
            Bold::try_from(inlines[0].clone()).unwrap(),
            Bold {
                inner: vec![Plain {
                    content: "bold1".to_string(),
                }
                .into()],
            },
            "Bold1 not correctly parsed."
        );
        assert_eq!(
            inlines[1],
            Inline::Whitespace(Whitespace {}),
            "Whitespace between bolds not correctly parsed."
        );
        assert_eq!(
            Bold::try_from(inlines[2].clone()).unwrap(),
            Bold {
                inner: vec![Plain {
                    content: "bold2".to_string(),
                }
                .into()],
            },
            "Bold2 not correctly parsed."
        );

        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_strikethrough_in_unclosed_bold() {
        let symbols = unimarkup_commons::scanner::scan_str("**~~strikethrough~~");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

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

        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_keyword_in_verbatim() {
        let symbols = unimarkup_commons::scanner::scan_str("**`**still-bold`");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        assert_eq!(
            inlines.len(),
            1,
            "Parser did not return one inline element."
        );

        assert_eq!(
            Bold::try_from(inlines[0].clone()).unwrap(),
            Bold {
                inner: vec![Verbatim {
                    inner: vec![Plain {
                        content: "**still-bold".to_string(),
                    }
                    .into()],
                }
                .into()],
            },
            "Keyword inside verbatim not correctly parsed."
        );

        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }

    #[test]
    fn parse_textbox_with_inner_bold() {
        let symbols = unimarkup_commons::scanner::scan_str("[**bold]");
        let mut sym_iter = SymbolIterator::from(&*symbols);

        let inlines = InlineParser::default().parse(&mut sym_iter);

        assert_eq!(
            inlines.len(),
            1,
            "Parser returned more than one inline element."
        );
        assert_eq!(
            inlines.first().unwrap(),
            &Inline::TextBox(TextBox {
                inner: vec![Bold {
                    inner: vec![Plain {
                        content: "bold".to_string()
                    }
                    .into()]
                }
                .into()]
            }),
            "Textbox with inner bold not correctly parsed."
        );
        assert_eq!(sym_iter.next(), None, "Iterator not fully consumed.");
    }
}
