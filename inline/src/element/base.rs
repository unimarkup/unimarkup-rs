use unimarkup_commons::lexer::{position::Position, span::Span};

use crate::tokenize::token::{InlineToken, InlineTokenKind};

use super::Inline;

macro_rules! base_inlines {
    ($($element:ident$( has $content:ident: $content_type:ty)?),+) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $element {
                $(
                    $content: $content_type,
                )?
                start: Position,
                end: Position,
            }

            impl From<$element> for Inline {
                fn from(value: $element) -> Self {
                    Inline::$element(value)
                }
            }

            impl $element {
                pub fn new($($content: $content_type, )?start: Position, end: Position) -> Self {
                    Self { $($content, )?start, end }
                }

                $(
                    pub fn $content(&self) -> &$content_type {
                        &self.$content
                    }
                )?

                pub fn start(&self) -> Position {
                    self.start
                }

                pub fn end(&self) -> Position {
                    self.end
                }

                pub fn span(&self) -> Span {
                    Span {
                        start: self.start,
                        end: self.end,
                    }
                }
            }
        )+
    }
}

base_inlines!(
    Plain has content: String,
    EscapedPlain has content: String,
    EscapedWhitespace has space: String,
    Whitespace,
    Newline,
    EscapedNewline
);

impl Plain {
    pub fn push_str(&mut self, s: &str) {
        self.content.push_str(s);
    }
}

impl<'input> From<InlineToken<'input>> for Inline {
    fn from(value: InlineToken<'input>) -> Self {
        match value.kind {
            InlineTokenKind::Newline => Inline::Newline(Newline::new(value.start, value.end)),
            InlineTokenKind::EscapedNewline => {
                Inline::EscapedNewline(EscapedNewline::new(value.start, value.end))
            }
            InlineTokenKind::Whitespace => {
                Inline::Whitespace(Whitespace::new(value.start, value.end))
            }
            InlineTokenKind::EscapedWhitespace => Inline::EscapedWhitespace(
                EscapedWhitespace::new(value.as_str().to_string(), value.start, value.end),
            ),
            InlineTokenKind::EscapedPlain => Inline::EscapedPlain(EscapedPlain::new(
                value.as_str().to_string(),
                value.start,
                value.end,
            )),

            // All other tokens are either created in parser, or did not resolve to an element => take as plain
            _ => Inline::Plain(Plain::new(
                value.as_str().to_string(),
                value.start,
                value.end,
            )),
        }
    }
}
