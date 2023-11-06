//! Inline parser

use unimarkup_commons::{
    lexer::token::iterator::{IteratorEndFn, IteratorPrefixFn, TokenIterator},
    parsing::InlineContext,
};

use crate::{
    element::Inline,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind},
};

/// Parser function type for inline element parsing.
pub(crate) type InlineParserFn =
    for<'s, 'i> fn(InlineParser<'s, 'i>) -> (InlineParser<'s, 'i>, Option<Inline>);

/// Creates inline elements using the given token iterator.
pub fn parse_inlines<'slice, 'input>(
    token_iter: TokenIterator<'slice, 'input>,
    context: InlineContext,
    prefix_match: Option<IteratorPrefixFn>,
    end_match: Option<IteratorEndFn>,
) -> (TokenIterator<'slice, 'input>, InlineContext, ParsedInlines) {
    let scoped_iter: TokenIterator<'slice, 'input> =
        token_iter.new_scope_root(prefix_match, end_match);

    let mut inline_parser = InlineParser {
        iter: InlineTokenIterator::from(scoped_iter),
        context,
    };
    let (updated_parser, inlines) = InlineParser::parse(inline_parser);

    let parsed_inlines = ParsedInlines {
        inlines,
        end_reached: updated_parser.iter.end_reached(),
        prefix_mismatch: updated_parser.iter.prefix_mismatch(),
    };
    inline_parser = updated_parser.unfold();

    (
        inline_parser.iter.into(),
        inline_parser.context,
        parsed_inlines,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInlines {
    inlines: Vec<Inline>,
    end_reached: bool,
    prefix_mismatch: bool,
}

impl ParsedInlines {
    pub fn to_inlines(self) -> Vec<Inline> {
        self.inlines
    }

    pub fn end_reached(&self) -> bool {
        self.end_reached
    }

    pub fn prefix_mismatch(&self) -> bool {
        self.prefix_mismatch
    }
}

#[derive(Debug)]
pub(crate) struct InlineParser<'slice, 'input> {
    pub iter: InlineTokenIterator<'slice, 'input>,
    pub context: InlineContext,
}

impl<'slice, 'input> InlineParser<'slice, 'input> {
    pub(crate) fn parse(mut parser: Self) -> (Self, Vec<Inline>) {
        let mut inlines = Vec::default();
        let mut format_closes = false;

        #[cfg(debug_assertions)]
        let mut curr_len = parser.iter.max_len();

        parser.iter.reset_peek();

        'outer: while let Some(kind) = parser.iter.peek_kind() {
            if kind == InlineTokenKind::Eoi {
                break 'outer;
            }

            let parser_fn_opt = if (!parser.context.flags.logic_only
                && kind.is_scoped_format_keyword())
                || kind.is_open_parenthesis()
            {
                get_scoped_parser(kind, parser.context.flags.logic_only)
            } else if !parser.context.flags.logic_only && kind.is_format_keyword() {
                // An open format closes => unwrap to closing format element
                // closing token is not consumed here => the element parser needs this info
                if parser.iter.format_closes(kind) {
                    format_closes = true;
                    break 'outer;
                } else if !parser.iter.format_is_open(kind) {
                    get_format_parser(kind)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(parser_fn) = parser_fn_opt {
                let checkpoint = parser.iter.checkpoint();
                let (updated_parser, inline_opt) = parser_fn(parser);
                parser = updated_parser;
                match inline_opt {
                    Some(inline) => {
                        inlines.push(inline);
                        continue 'outer;
                    }
                    None => {
                        let success = parser.iter.rollback(checkpoint);
                        debug_assert!(
                            success,
                            "Rollback was not successful for checkpoint '{:?}'",
                            checkpoint
                        )
                    }
                }
            }

            let (updated_parser, updated_inlines) =
                crate::element::base::parse_base(parser, inlines);

            parser = updated_parser;
            inlines = updated_inlines;

            #[cfg(debug_assertions)]
            {
                assert!(
                    parser.iter.max_len() < curr_len,
                    "Parser consumed no symbol in iteration."
                );
                curr_len = parser.iter.max_len();
            }
        }

        if !format_closes {
            // To consume tokens in end matching, but do not consume closing formatting tokens
            let _ = parser.iter.next();
        }

        (parser, inlines)
    }

    pub fn nest_scoped(mut self, end_match: Option<IteratorEndFn>) -> Self {
        self.iter = self.iter.nest_scoped(end_match);
        self
    }

    pub fn unfold(mut self) -> Self {
        self.iter = self.iter.unfold();
        self
    }
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

    use crate::{inline_parser::InlineParser, tokenize::iterator::InlineTokenIterator};

    #[test]
    fn dummy_for_debugging() {
        let tokens = unimarkup_commons::lexer::token::lex_str("`a`");
        let mut inline_parser = InlineParser {
            iter: InlineTokenIterator::from(TokenIterator::from(&*tokens)),
            context: InlineContext::default(),
        };

        let (updated_parser, inlines) = InlineParser::parse(inline_parser);
        inline_parser = updated_parser;

        // dbg!(&inlines);

        assert!(!inlines.is_empty(), "No inlines created.");
        assert_eq!(
            inline_parser.iter.next(),
            None,
            "Iterator not fully consumed."
        );
    }
}
