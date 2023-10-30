use unimarkup_commons::{lexer::position::Position, parsing::Context};

use crate::{
    element::InlineElement,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind, InlineToken},
};

use super::{
    substitution::{DirectUri, ImplicitSubstitution},
    Inline,
};

pub(crate) fn parse_base(
    input: &mut InlineTokenIterator,
    context: &mut Context,
    inlines: &mut Vec<Inline>,
) {
    let mut next = input.next().expect("Peeked symbol before.");
    let kind = next.kind;

    if kind.is_keyword() {
        // Ambiguous token may be split to get possible valid partial token
        crate::element::formatting::ambiguous::ambiguous_split(input, &mut next);

        // If keyword was not handled above => convert token to plain
        next.kind = InlineTokenKind::Plain;
        input.set_prev_token(next); // update prev token, because next changed afterwards
    } else if (context.keep_spaces && kind == InlineTokenKind::Whitespace)
        || (context.macros_only
            && matches!(
                kind,
                InlineTokenKind::Comment { .. } | InlineTokenKind::ImplicitSubstitution(_)
            ))
    {
        // Only escapes, newlines and macros remain as is in macro only mode
        // This is used for example in verbatim context
        next.kind = InlineTokenKind::Plain;
        input.set_prev_token(next); // update prev token, because next changed afterwards
    }

    match inlines.last_mut() {
        Some(last) => match last {
            Inline::Plain(plain) if next.kind == InlineTokenKind::Plain => {
                plain.push_token(next);
            }
            _ => inlines.push(next.into()),
        },
        None => inlines.push(next.into()),
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
            InlineTokenKind::ImplicitSubstitution(kind) => Inline::ImplicitSubstitution(
                ImplicitSubstitution::new(kind, value.start, value.end),
            ),
            InlineTokenKind::Directuri => Inline::DirectUri(DirectUri::new(
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

            $(
                impl InlineElement for $element {
                    fn to_plain_string(&self) -> String {
                        self.$content.clone()
                    }

                    fn start(&self) -> Position {
                        self.start
                    }

                    fn end(&self) -> Position {
                        self.end
                    }
                }
            )?

            impl $element {
                pub fn new($($content: $content_type, )?start: Position, end: Position) -> Self {
                    Self { $($content, )?start, end }
                }

                $(
                    pub fn $content(&self) -> &$content_type {
                        &self.$content
                    }
                )?
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
    pub(crate) fn push_token(&mut self, token: InlineToken<'_>) {
        self.end = token.end;
        self.content.push_str(token.as_str());
    }
}

macro_rules! element_without_content {
    ($($element:ident),+) => {
        $(
            impl InlineElement for $element {
                fn to_plain_string(&self) -> String {
                    InlineTokenKind::$element.as_str().to_string()
                }

                fn start(&self) -> Position {
                    self.start
                }

                fn end(&self) -> Position {
                    self.end
                }
            }
        )+
    };
}

element_without_content!(Whitespace, Newline, EscapedNewline);
