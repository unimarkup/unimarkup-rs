//! Inline parser

use unimarkup_commons::lexer::token::iterator::{
    IteratorEndFn, IteratorPrefixFn, PeekingNext, TokenIterator,
};

use crate::{
    element::{formatting::OpenFormatMap, Inline},
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
    inline_parser = updated_parser.unfold(OpenFormatMap::default());

    (
        inline_parser.iter.into(),
        inline_parser.context,
        parsed_inlines,
    )
}

/// Struct to return parsed inline elements,
/// and additional infos about the inline parser state at the end of the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInlines {
    inlines: Vec<Inline>,
    end_reached: bool,
    prefix_mismatch: bool,
}

impl ParsedInlines {
    /// Convert [`ParsedInlines`] to inlines.
    pub fn to_inlines(self) -> Vec<Inline> {
        self.inlines
    }

    /// Returns `true` if the inline parser reached its end.
    pub fn end_reached(&self) -> bool {
        self.end_reached
    }

    /// Returns `true` if the inline parser had a prefix mismatch.
    pub fn prefix_mismatch(&self) -> bool {
        self.prefix_mismatch
    }
}

/// Context to help with parsing Unimarkup inline content.
#[derive(Debug, Default, Clone)]
pub struct InlineContext {
    pub flags: InlineContextFlags,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InlineContextFlags {
    /// Flag to indicate that only escaped graphemes and logic elements are allowed besides plain content.
    pub logic_only: bool,
    /// Flag to indicate that multiple contiguous whitespaces must not be combined.
    pub keep_whitespaces: bool,
    /// Flag to indicate that a newline must be explicitly kept, and not converted to one space.
    pub keep_newline: bool,
    /// Flag to indicate if implicit substitutions are allowed in the current context
    pub allow_implicits: bool,
}

/// The inline parser containing the [`InlineTokenIterator`],
/// and the [`InlineContext`] used to parse Unimarkup content to create inline elements.
#[derive(Debug)]
pub(crate) struct InlineParser<'slice, 'input> {
    pub iter: InlineTokenIterator<'slice, 'input>,
    pub context: InlineContext,
}

impl<'slice, 'input> InlineParser<'slice, 'input> {
    /// The main parser for inline elements.
    pub(crate) fn parse(mut parser: Self) -> (Self, Vec<Inline>) {
        let mut inlines = Vec::default();
        let mut format_closes = false;

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
                    if kind == InlineTokenKind::DoubleQuote {
                        let next_is_quote = parser
                            .iter
                            .peeking_next(|t| t.kind == InlineTokenKind::DoubleQuote)
                            .is_some();
                        let next_next_is_quote = parser
                            .iter
                            .peeking_next(|t| t.kind == InlineTokenKind::DoubleQuote)
                            .is_some();
                        if next_is_quote && !next_next_is_quote {
                            // Consume first quote
                            parser.iter.next();
                            format_closes = true;
                            break 'outer;
                        } else {
                            parser.iter.reset_peek();
                            // Otherwhise, no quote element
                            None
                        }
                    } else {
                        format_closes = true;
                        break 'outer;
                    }
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
                            "Inline rollback was not successful at '{:?}'",
                            parser.iter.peek()
                        )
                    }
                }
            }

            let (updated_parser, updated_inlines) =
                crate::element::base::parse_base(parser, inlines);

            parser = updated_parser;
            inlines = updated_inlines;
        }

        if !format_closes {
            // To consume tokens in end matching, but do not consume closing formatting tokens
            let _ = parser.iter.next();
        }

        (parser, inlines)
    }

    /// Create an inline parser that has this parser as parent.
    /// Returns the nested parser, and the [`OpenFormatMap`] of the outer scope.
    /// This [`OpenFormatMap`] must be used when calling `unfold()` to get correct inline formatting.
    pub fn nest_scoped(mut self, end_match: Option<IteratorEndFn>) -> (Self, OpenFormatMap) {
        let (scoped_iter, outer_open_formats) = self.iter.nest_scoped(end_match);
        self.iter = scoped_iter;

        (self, outer_open_formats)
    }

    /// Returns the parent parser if this parser is nested.
    /// Overrides the internal [`OpenFormatMap`] with the given one.
    pub fn unfold(mut self, outer_open_formats: OpenFormatMap) -> Self {
        self.iter = self.iter.unfold(outer_open_formats);
        self
    }
}

/// Returns the parser that is able to create an inline format element from the given kind.
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
        | InlineTokenKind::Overline => Some(crate::element::formatting::parse_distinct_format),
        InlineTokenKind::DoubleQuote => Some(crate::element::formatting::parse_quote_format),
        _ => None,
    }
}

/// Returns the parser that is able to create an inline scoped parser from the given kind.
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
    use unimarkup_commons::lexer::token::iterator::TokenIterator;

    use crate::{
        parser::{InlineContext, InlineParser},
        tokenize::iterator::InlineTokenIterator,
    };

    #[test]
    fn dummy_for_debugging() {
        let tokens = unimarkup_commons::lexer::token::lex_str("[Simple textbox]");
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
