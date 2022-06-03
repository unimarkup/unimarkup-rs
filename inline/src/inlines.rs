use crate::{Span, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Bold(NestedContent),
    Italic(NestedContent),
    Underline(NestedContent),
    Subscript(NestedContent),
    Superscript(NestedContent),
    Overline(NestedContent),
    Strikethrough(NestedContent),
    Highlight(NestedContent),
    Verbatim(PlainContent),
    Quote(NestedContent),
    Math(NestedContent),
    Parens(PlainContent),
    TextGroup(NestedContent),
    Attributes(NestedContent),
    Newline(PlainContent),
    Whitespace(PlainContent),
    Plain(PlainContent),
}

impl Inline {
    pub fn into_inner(self) -> InlineContent {
        match self {
            Inline::Verbatim(plain_content)
            | Inline::Parens(plain_content)
            | Inline::Newline(plain_content)
            | Inline::Whitespace(plain_content)
            | Inline::Plain(plain_content) => InlineContent::Plain(plain_content),

            Inline::Bold(nested_content)
            | Inline::Italic(nested_content)
            | Inline::Underline(nested_content)
            | Inline::Subscript(nested_content)
            | Inline::Superscript(nested_content)
            | Inline::Overline(nested_content)
            | Inline::Strikethrough(nested_content)
            | Inline::Highlight(nested_content)
            | Inline::Quote(nested_content)
            | Inline::Math(nested_content)
            | Inline::TextGroup(nested_content)
            | Inline::Attributes(nested_content) => InlineContent::Nested(nested_content),
        }
    }

    pub fn as_str(&self) -> &str {
        self.as_ref().as_str()
    }

    pub fn content_len(&self) -> usize {
        match self {
            Inline::Verbatim(plain_content)
            | Inline::Parens(plain_content)
            | Inline::Newline(plain_content)
            | Inline::Whitespace(plain_content)
            | Inline::Plain(plain_content) => plain_content.content_len(),

            Inline::Bold(nested_content)
            | Inline::Italic(nested_content)
            | Inline::Underline(nested_content)
            | Inline::Subscript(nested_content)
            | Inline::Superscript(nested_content)
            | Inline::Overline(nested_content)
            | Inline::Strikethrough(nested_content)
            | Inline::Highlight(nested_content)
            | Inline::Quote(nested_content)
            | Inline::Math(nested_content)
            | Inline::TextGroup(nested_content)
            | Inline::Attributes(nested_content) => nested_content.content_len(),
        }
    }
}

impl AsRef<InlineContent> for Inline {
    fn as_ref(&self) -> &InlineContent {
        match self {
            Inline::Verbatim(content)
            | Inline::Parens(content)
            | Inline::Newline(content)
            | Inline::Whitespace(content)
            | Inline::Plain(content) => &InlineContent::Plain(*content),

            Inline::Bold(content)
            | Inline::Italic(content)
            | Inline::Underline(content)
            | Inline::Subscript(content)
            | Inline::Superscript(content)
            | Inline::Overline(content)
            | Inline::Strikethrough(content)
            | Inline::Highlight(content)
            | Inline::Quote(content)
            | Inline::Math(content)
            | Inline::TextGroup(content)
            | Inline::Attributes(content) => &InlineContent::Nested(*content),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineContent {
    Plain(PlainContent),
    Nested(NestedContent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedContent {
    content: Vec<Inline>,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlainContent {
    content: String,
    span: Span,
}

impl NestedContent {
    pub fn content_len(&self) -> usize {
        self.content.iter().map(|inline| inline.content_len()).sum()
    }
}

impl PlainContent {
    pub fn as_str(&self) -> &str {
        &self.content
    }

    pub fn content_len(&self) -> usize {
        self.content.len()
    }
}

impl InlineContent {
    pub fn try_flatten(&mut self) {
        todo!()
    }

    pub fn prepend(&mut self, content: InlineContent) {
        todo!()
    }

    pub fn append_inline(&mut self, inline: Inline) {
        todo!()
    }

    pub fn append(&mut self, content: InlineContent) {
        todo!()
    }

    pub fn from_token_as_plain(token: Token) -> Self {
        let content = String::from(token.as_str());
        let span = token.span();

        InlineContent::Plain(PlainContent { content, span })
    }

    pub fn as_str(&self) -> &str {
        match self {
            InlineContent::Plain(content) => content.as_str(),
            InlineContent::Nested(nested_content) => &nested_content.to_string(),
        }
    }

    pub fn content_len(&self) -> usize {
        match self {
            InlineContent::Plain(plain_content) => plain_content.content_len(),
            InlineContent::Nested(nested_inlines) => nested_inlines.content_len(),
        }
    }
}

impl From<NestedContent> for InlineContent {
    fn from(content: NestedContent) -> Self {
        Self::Nested(content)
    }
}

impl From<PlainContent> for InlineContent {
    fn from(content: PlainContent) -> Self {
        Self::Plain(content)
    }
}

impl From<InlineContent> for Option<PlainContent> {
    fn from(content: InlineContent) -> Option<PlainContent> {
        match content {
            InlineContent::Plain(plain_content) => Some(plain_content),
            InlineContent::Nested(nested_inlines) => {
                if nested_inlines.content.len() == 0 {
                    None
                } else {
                    let content_len = content.content_len();
                    let content: String = String::with_capacity(content_len);

                    for inline in nested_inlines.content {
                        content.push_str(inline.as_str());
                    }

                    Some(PlainContent {
                        content,
                        span: nested_inlines.span,
                    })
                }
            }
        }
    }
}

impl NestedContent {
    pub fn len(&self) -> usize {
        self.content.len()
    }
}
