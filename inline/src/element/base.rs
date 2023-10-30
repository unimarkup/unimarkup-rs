use unimarkup_commons::{lexer::position::Position, parsing::InlineContext};

use crate::{
    element::InlineElement,
    tokenize::{iterator::InlineTokenIterator, kind::InlineTokenKind, InlineToken},
};

use super::{substitution::DirectUri, Inline};

pub(crate) fn parse_base(
    input: &mut InlineTokenIterator,
    context: &mut InlineContext,
    inlines: &mut Vec<Inline>,
) {
    let mut next = input.next().expect("Peeked symbol before.");
    let kind = next.kind;

    if kind.is_keyword() {
        // Ambiguous token may be split to get possible valid partial token
        crate::element::formatting::ambiguous::ambiguous_split(input, &mut next);

        // Keyword did not lead to inline element in inline parser => convert token to plain
        next.kind = InlineTokenKind::Plain;
        input.set_prev_token(next); // update prev token, because next changed afterwards
    } else if context.flags.keep_whitespaces && kind == InlineTokenKind::Whitespace {
        // Treating whitespace as plain will preserve the original whitespace, and not compress it to a single space.
        next.kind = InlineTokenKind::Plain;
        // Previous token is not updated, because format opening/closing validation needs whitespace information
    } else if context.flags.logic_only
        && matches!(
            kind,
            InlineTokenKind::ImplicitSubstitution(_) | InlineTokenKind::Directuri
        )
    {
        next.kind = InlineTokenKind::Plain;

        #[cfg(debug_assertions)]
        panic!(
            "Kind '{:?}' in logic_only is not supported. Use token iterator's `ignore_implicits()` before entering *logic only* scope.",
            kind
        );
    }

    match inlines.last_mut() {
        Some(last) => match last {
            Inline::Plain(plain)
                if matches!(
                    next.kind,
                    InlineTokenKind::Plain
                        | InlineTokenKind::Whitespace
                        | InlineTokenKind::ImplicitSubstitution(_)
                ) =>
            {
                plain.push_token(next);
            }
            _ => inlines.push(to_inline(input, context, next)),
        },
        None => inlines.push(to_inline(input, context, next)),
    }
}

fn to_inline(
    token_iter: &mut InlineTokenIterator,
    context: &InlineContext,
    token: InlineToken<'_>,
) -> Inline {
    let prev_kind = token_iter.prev_kind();

    match token.kind {
        InlineTokenKind::Newline => {
            if context.flags.keep_newline {
                Inline::ImplicitNewline(Newline::new(token.start, token.end))
            } else {
                Inline::Newline(Newline::new(token.start, token.end))
            }
        }
        InlineTokenKind::EscapedNewline => {
            Inline::EscapedNewline(EscapedNewline::new(token.start, token.end))
        }

        InlineTokenKind::EscapedWhitespace => Inline::EscapedWhitespace(EscapedWhitespace::new(
            token.as_str().to_string(),
            token.start,
            token.end,
        )),
        InlineTokenKind::EscapedPlain => Inline::EscapedPlain(EscapedPlain::new(
            token.as_str().to_string(),
            token.start,
            token.end,
        )),
        InlineTokenKind::Directuri => Inline::DirectUri(DirectUri::new(
            token.as_str().to_string(),
            token.start,
            token.end,
        )),

        // No plain content before whitespace => could not merge with previous => creates new plain
        InlineTokenKind::Whitespace => {
            debug_assert!(
                !context.flags.keep_whitespaces,
                "Whitespace was not converted to `Plain` to preserve whitespace."
            );

            match prev_kind {
                Some(InlineTokenKind::Newline) => {
                    // Ignore whitespaces after newline, because newline already represents one space
                    Inline::Plain(Plain::new(String::default(), token.start, token.end))
                }
                _ if matches!(
                    token_iter.peek_kind(),
                    Some(InlineTokenKind::Newline) | Some(InlineTokenKind::EscapedNewline)
                ) =>
                {
                    // Ignore whitespaces before newline, because newline already represents one space
                    Inline::Plain(Plain::new(String::default(), token.start, token.end))
                }
                _ => Inline::Plain(Plain::new(
                    token.as_str().to_string(),
                    token.start,
                    token.end,
                )),
            }
        }
        // No plain content before subst => could not merge with previous => creates new plain
        InlineTokenKind::ImplicitSubstitution(subst) => Inline::Plain(Plain::new(
            subst.subst().to_string(),
            token.start,
            token.end,
        )),
        _ => {
            debug_assert_eq!(
                token.kind,
                InlineTokenKind::Plain,
                "Inline kind '{:?}' was not set to `Plain` before converting to `Inline`.",
                token.kind
            );

            Inline::Plain(Plain::new(
                token.as_str().to_string(),
                token.start,
                token.end,
            ))
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
    Newline,
    EscapedNewline
);

impl Plain {
    pub(crate) fn push_token(&mut self, token: InlineToken<'_>) {
        let content = if let InlineTokenKind::ImplicitSubstitution(subst) = token.kind {
            subst.subst()
        } else {
            debug_assert_eq!(
                token.kind,
                InlineTokenKind::Plain,
                "Tried to push kind '{:?}' to plain inline.",
                token.kind
            );
            token.as_str()
        };

        self.end = token.end;
        self.content.push_str(content);
    }
}

macro_rules! element_without_content {
    ($($element:ident),+) => {
        $(
            impl $element {
                pub fn as_str(&self) -> &'static str {
                    InlineTokenKind::$element.as_str()
                }
            }

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

element_without_content!(Newline, EscapedNewline);
