//! Contains formatting elements like [`Bold`], [`Italic`], [`Underline`], ...

use unimarkup_commons::lexer::{position::Position, token::iterator::PeekingNext};

use crate::{element::InlineElement, parser::InlineParser, tokenize::kind::InlineTokenKind};

use super::Inline;

pub mod ambiguous;
pub mod scoped;

/// Parses formatting elements that have distinct keywords assigned to them.
/// e.g. [`Strikethrough`] or [`Quote`]
pub(crate) fn parse_distinct_format<'s, 'i>(
    mut parser: InlineParser<'s, 'i>,
) -> (InlineParser<'s, 'i>, Option<Inline>) {
    let open_token_opt = parser.iter.peeking_next(|_| true);
    if open_token_opt.is_none() {
        return (parser, None);
    }

    let open_token = open_token_opt.expect("Checked above to be not None.");

    // No need to check for correct opening format, because parser is only assigned for valid opening tokens.
    if parser.iter.peek_kind().map_or(true, |t| t.is_space()) {
        return (parser, None);
    }

    parser.iter.next(); // consume open token => now it will lead to Some(inline)

    parser.iter.open_format(&open_token.kind);

    let (updated_parser, inner) = InlineParser::parse(parser);
    parser = updated_parser;

    let attributes = None;
    let mut implicit_end = true;

    // Only consuming token on open/close match, because closing token might be reserved for an outer open format.
    let end = if let Some(close_token) = parser.iter.peek() {
        if close_token.kind == open_token.kind {
            parser
                .iter
                .next()
                .expect("Peeked before, so `next` must return Some.");
            implicit_end = false;

            //TODO: check for optional attributes here
            close_token.end
        } else {
            close_token.start
        }
    } else {
        parser
            .iter
            .prev_token()
            .expect("Previous token must exist here, because format was opened.")
            .end
    };

    parser.iter.close_format(&open_token.kind);

    (
        parser,
        Some(to_formatting(
            open_token.kind,
            inner,
            attributes,
            open_token.start,
            end,
            implicit_end,
        )),
    )
}

macro_rules! inline_formats {
    ($($format:ident),+) => {
        $(
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $format {
            inner: Vec<Inline>,
            attributes: Option<Vec<Inline>>,
            start: Position,
            end: Position,
            implicit_end: bool,
        }

        impl From<$format> for Inline {
            fn from(value: $format) -> Self {
                Inline::$format(value)
            }
        }

        impl InlineElement for $format {
            fn as_unimarkup(&self) -> String {
                format!("{}{}{}", InlineTokenKind::$format.as_str(), self.inner.as_unimarkup(), if self.implicit_end {""} else {InlineTokenKind::$format.as_str()})
            }

            fn start(&self) -> Position {
                self.start
            }

            fn end(&self) -> Position {
                self.end
            }
        }

        impl $format {
            pub fn new(
                inner: Vec<Inline>,
                attributes: Option<Vec<Inline>>,
                start: Position,
                end: Position,
                implicit_end: bool,
            ) -> Self {
                Self {
                    inner,
                    attributes,
                    start,
                    end,
                    implicit_end,
                }
            }

            pub fn inner(&self) -> &Vec<Inline> {
                &self.inner
            }

            pub fn attributes(&self) -> Option<&Vec<Inline>> {
                self.attributes.as_ref()
            }

            pub fn implicit_end(&self) -> bool {
                self.implicit_end
            }
        })+
    };
}

macro_rules! format_to_inline{
    ($($format:ident),+) => {
        pub(crate) fn to_formatting(
            kind: InlineTokenKind,
            inner: Vec<Inline>,
            attributes: Option<Vec<Inline>>,
            start: Position,
            end: Position,
            implicit_end: bool,
        ) -> Inline {
            match kind {
            $(
                InlineTokenKind::$format => $format {
                    inner,
                    attributes,
                    start,
                    end,
                    implicit_end,
                }
                .into(),
            )+
            _ => {
                    #[cfg(debug_assertions)]
                    panic!(
                        "Tried to create inline format from non-format kind '{:?}'",
                        kind
                    );

                    #[cfg(not(debug_assertions))]
                    $crate::element::base::Plain::new(
                        "".to_string(),
                        start,
                        end,
                    )
                    .into()
                },
            }
        }
    }
}

inline_formats!(
    Bold,
    Italic,
    Underline,
    Subscript,
    Superscript,
    Strikethrough,
    Highlight,
    Overline,
    Verbatim,
    Quote,
    Math
);

format_to_inline!(
    Bold,
    Italic,
    Underline,
    Subscript,
    Superscript,
    Strikethrough,
    Highlight,
    Overline,
    Verbatim,
    Quote,
    Math
);

const BOLD_INDEX: usize = 0;
const ITALIC_INDEX: usize = 1;
const UNDERLINE_INDEX: usize = 2;
const SUBSCRIPT_INDEX: usize = 3;
const SUPERSCRIPT_INDEX: usize = 4;
const STRIKETHROUGH_INDEX: usize = 5;
const HIGHLIGHT_INDEX: usize = 6;
const OVERLINE_INDEX: usize = 7;
const QUOTE_INDEX: usize = 8;
const NR_OF_UNSCOPED_FORMATS: usize = 9;

/// Type used to keep track of open formats that do not open their own scope.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OpenFormatMap([bool; NR_OF_UNSCOPED_FORMATS]);

impl OpenFormatMap {
    pub(crate) fn is_open(&self, index: usize) -> bool {
        self.0[index]
    }

    pub(crate) fn open(&mut self, index: usize) {
        self.0[index] = true;
    }

    pub(crate) fn close(&mut self, index: usize) {
        self.0[index] = false;
    }
}

/// Returns the index in the open format map for the given unscoped format.
pub(crate) fn map_index(kind: &InlineTokenKind) -> usize {
    match kind {
        InlineTokenKind::Bold => BOLD_INDEX,
        InlineTokenKind::Italic => ITALIC_INDEX,
        InlineTokenKind::Underline => UNDERLINE_INDEX,
        InlineTokenKind::Subscript => SUBSCRIPT_INDEX,
        InlineTokenKind::Superscript => SUPERSCRIPT_INDEX,
        InlineTokenKind::Strikethrough => STRIKETHROUGH_INDEX,
        InlineTokenKind::Highlight => HIGHLIGHT_INDEX,
        InlineTokenKind::Overline => OVERLINE_INDEX,
        InlineTokenKind::Quote => QUOTE_INDEX,
        _ => {
            #[cfg(debug_assertions)]
            panic!("Kind '{:?}' has no index in open format map.", kind);

            #[cfg(not(debug_assertions))]
            0
        }
    }
}
