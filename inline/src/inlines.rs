use crate::{Span, Token, TokenKind};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PlainInline {
    pub(crate) content: String,
    pub(crate) span: Span,
}

impl PlainInline {
    pub fn as_str(&self) -> &str {
        &self.content
    }
}

impl From<Token> for PlainInline {
    fn from(token: Token) -> Self {
        Self {
            content: String::from(token.as_str()),
            span: token.span(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineContent {
    Plain(PlainInline),
    Nested(Vec<Inline>),
}

impl InlineContent {
    pub fn append(&mut self, other: Self) {
        let mut first_part = match self {
            InlineContent::Plain(plain_inline) => {
                let inner = std::mem::take(plain_inline);
                vec![Inline::from(inner)]
            }
            InlineContent::Nested(existing_inlines) => std::mem::take(existing_inlines),
        };

        let mut second_part = match other {
            InlineContent::Plain(plain_inline) => vec![Inline::from(plain_inline)],
            InlineContent::Nested(existing_inlines) => existing_inlines,
        };

        first_part.append(&mut second_part);

        *self = InlineContent::Nested(first_part);
    }

    pub fn append_inline(&mut self, inline: Inline) {
        self.append(InlineContent::Nested(vec![inline]));
    }

    pub fn prepend(&mut self, other: Self) {
        let mut self_content = match self {
            InlineContent::Plain(plain_inline) => {
                let inner = std::mem::take(plain_inline);
                vec![Inline::from(inner)]
            }
            InlineContent::Nested(existing_inlines) => std::mem::take(existing_inlines),
        };

        let mut other_content = match other {
            InlineContent::Plain(plain_inline) => vec![Inline::from(plain_inline)],
            InlineContent::Nested(existing_inlines) => existing_inlines,
        };

        other_content.append(&mut self_content);

        *self = InlineContent::Nested(other_content);
    }

    pub fn prepend_inline(&mut self, inline: Inline) {
        self.prepend(inline.into_inner());
    }

    pub fn unwrap_plain(self) -> PlainInline {
        match self {
            InlineContent::Plain(plain_inline) => plain_inline,
            InlineContent::Nested(_) => panic!("Tried to unwrap plain on nested inline content."),
        }
    }

    pub fn try_flatten(&mut self) {
        match self {
            InlineContent::Plain(_) => (), // do nothing, already plain
            InlineContent::Nested(ref mut content) => {
                if content.is_empty() {
                    // do nothing
                    return;
                }

                if content
                    .iter()
                    .all(|inline| matches!(inline.kind, InlineKind::Plain))
                {
                    let first_inline = content.remove(0);
                    let start = first_inline.span().start();
                    let mut end = first_inline.span().end();

                    // this is checked, it is guaranteed that it is plain variant
                    let mut inline_content = first_inline.inner.unwrap_plain();

                    for inline in content {
                        end = inline.span().end();

                        match &inline.inner {
                            InlineContent::Plain(inner_content) => {
                                inline_content.content.push_str(&inner_content.content);
                            }
                            InlineContent::Nested(_) => unreachable!(
                                "Already checked, every inline contains plain content at this time"
                            ),
                        }
                    }

                    inline_content.span = Span::from((start, end));
                    *self = InlineContent::Plain(inline_content);
                }
            }
        }
    }
}

impl From<Token> for InlineContent {
    fn from(token: Token) -> Self {
        Self::Plain(PlainInline::from(token))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inline {
    pub(crate) inner: InlineContent,
    pub(crate) span: Span,
    pub(crate) kind: InlineKind,
}

impl Inline {
    pub fn new(span: Span, inner: InlineContent, kind: TokenKind) -> Self {
        Self {
            inner,
            span,
            kind: InlineKind::from(kind),
        }
    }

    pub fn into_inner(self) -> InlineContent {
        self.inner
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl From<PlainInline> for Inline {
    fn from(plain_inline: PlainInline) -> Self {
        Self {
            span: plain_inline.span,
            inner: InlineContent::Plain(plain_inline),
            kind: InlineKind::Plain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineKind {
    Bold,
    Italic,
    Underline,
    Subscript,
    Superscript,
    Overline,
    Strikethrough,
    Highlight,
    Verbatim,
    Quote,
    Math,
    Parens,
    TextGroup,
    Attributes,
    Newline,
    Whitespace,
    Plain,
}

impl From<TokenKind> for InlineKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Bold => Self::Bold,
            TokenKind::Italic => Self::Italic,
            TokenKind::Underline => Self::Underline,
            TokenKind::Subscript => Self::Subscript,
            TokenKind::Superscript => Self::Superscript,
            TokenKind::Overline => Self::Overline,
            TokenKind::Strikethrough => Self::Strikethrough,
            TokenKind::Highlight => Self::Highlight,
            TokenKind::Verbatim => Self::Verbatim,
            TokenKind::Quote => Self::Quote,
            TokenKind::Math => Self::Math,
            TokenKind::OpenParens | TokenKind::CloseParens => Self::Parens,
            TokenKind::OpenBracket | TokenKind::CloseBracket => Self::TextGroup,
            TokenKind::OpenBrace | TokenKind::CloseBrace => Self::Attributes,
            TokenKind::Newline => Self::Newline,
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Plain => Self::Plain,
            TokenKind::UnderlineSubscript => Self::Plain,
            TokenKind::ItalicBold => Self::Plain,
        }
    }
}
