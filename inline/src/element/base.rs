use unimarkup_commons::{
    lexer::{position::Position, Itertools},
    parsing::InlineContext,
};

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
    context: &mut InlineContext,
    inlines: &mut Vec<Inline>,
) {
    // This also helps to reset any possible peek while trying to parse special elements
    let mut next = input.next().expect("Peeked symbol in inline parser.");
    let kind = next.kind;

    if matches!(kind, InlineTokenKind::Whitespace) {
        // Previous token is not updated, because format opening/closing validation needs whitespace information
        // Compresses multiple contiguous whitespaces into one.
        if context.flags.keep_whitespaces {
            // Converting whitespace to plain will preserve content as is
            next.kind = InlineTokenKind::Plain;
        } else {
            let prev_kind = input.prev_kind();

            if let Some(last_whitespace) = input
                .peeking_take_while(|t| t.kind == InlineTokenKind::Whitespace)
                .last()
            {
                next.offset.end = last_whitespace.offset.end;
                next.end = last_whitespace.end
            }

            if matches!(
                prev_kind,
                Some(InlineTokenKind::Newline) | Some(InlineTokenKind::EscapedNewline)
            ) || matches!(
                input.peek_kind(),
                Some(InlineTokenKind::Newline) | Some(InlineTokenKind::EscapedNewline)
            ) {
                // Ignore whitespaces after newline, because newline already represents one space
                // Ignore whitespaces before newline, because newline already represents one space
                return;
            }
        }
    }

    if kind.is_keyword() {
        // Ambiguous token may be split to get possible valid partial token
        crate::element::formatting::ambiguous::ambiguous_split(input, &mut next);

        // Keyword did not lead to inline element in inline parser => convert token to plain
        next.kind = InlineTokenKind::Plain;
        input.set_prev_token(next); // update prev token, because next changed afterwards
    } else if !context.flags.allow_implicits
        && matches!(
            kind,
            InlineTokenKind::ImplicitSubstitution(_) | InlineTokenKind::Directuri
        )
    {
        next.kind = InlineTokenKind::Plain;
    }

    match inlines.last_mut() {
        Some(last) => match last {
            Inline::Plain(plain)
                if matches!(
                    next.kind,
                    InlineTokenKind::Plain | InlineTokenKind::Whitespace
                ) =>
            {
                plain.push_token(next);
            }
            _ => inlines.push(to_inline(context, next)),
        },
        None => inlines.push(to_inline(context, next)),
    }
}

fn to_inline(context: &InlineContext, token: InlineToken<'_>) -> Inline {
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
        InlineTokenKind::ImplicitSubstitution(subst) => {
            Inline::ImplicitSubstitution(ImplicitSubstitution::new(subst, token.start, token.end))
        }
        _ => {
            debug_assert!(
                matches!(
                    token.kind,
                    InlineTokenKind::Plain | InlineTokenKind::Whitespace
                ),
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
        debug_assert!(
            matches!(
                token.kind,
                InlineTokenKind::Plain | InlineTokenKind::Whitespace
            ),
            "Tried to push kind '{:?}' to plain inline.",
            token.kind
        );

        self.end = token.end;
        self.content.push_str(token.as_str());
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
