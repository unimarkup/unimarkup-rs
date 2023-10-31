use unimarkup_commons::{lexer::position::Position, parsing::InlineContext};

use crate::{
    element::InlineElement,
    inline_parser,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind},
};

use super::Inline;

pub mod ambiguous;
pub mod scoped;

pub(crate) fn parse_distinct_format(
    input: &mut InlineTokenIterator,
    context: &mut InlineContext,
) -> Option<Inline> {
    let open_token = input.next()?;

    // No need to check for correct opening format, because parser is only assigned for valid opening tokens.
    if input.peek_kind()?.is_space() {
        return None;
    }

    input.push_format(open_token.kind);

    let inner = inline_parser::parse(input, context);

    let attributes = None;
    let mut implicit_end = true;

    // Only consuming token on open/close match, because closing token might be reserved for an outer open format.
    let end = if let Some(close_token) = input.peek() {
        if close_token.kind == open_token.kind {
            input.next()?;
            implicit_end = false;

            //TODO: check for optional attributes here
            close_token.end
        } else {
            close_token.start
        }
    } else {
        input
            .prev_token()
            .expect("Previous token must exist here, because format was opened.")
            .end
    };

    input.pop_format(open_token.kind);
    Some(to_formatting(
        open_token.kind,
        inner,
        attributes,
        open_token.start,
        end,
        implicit_end,
    ))
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
            fn to_plain_string(&self) -> String {
                format!("{}{}{}", InlineTokenKind::$format.as_str(), self.inner.to_plain_string(), if self.implicit_end {""} else {InlineTokenKind::$format.as_str()})
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
