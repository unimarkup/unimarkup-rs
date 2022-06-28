use std::collections::VecDeque;

use crate::{Span, TokenKind};

mod content;

pub use content::*;

/// Representation of Unimarkup inline-formatted text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    /// Bold formatted content.
    Bold(NestedContent),

    /// Italic formatted content.
    Italic(NestedContent),

    /// Underlined content.
    Underline(NestedContent),

    /// Content in a subscript.   
    Subscript(NestedContent),

    /// Content in a superscript.
    Superscript(NestedContent),

    /// Overlined content.
    Overline(NestedContent),

    /// Content with a strikethrough.
    Strikethrough(NestedContent),

    /// Highlighted content.
    Highlight(NestedContent),

    /// Verbatim (monospaced) content.
    Verbatim(PlainContent),

    /// Content as a quotation.
    Quote(NestedContent),

    /// LaTeX-like math content.
    Math(NestedContent),

    /// Content inside a pair of parenthesis `()`.
    Parens(PlainContent),

    /// Content of a TextGroup `[]`.
    TextGroup(NestedContent),

    /// Unimarkup attributes for some content.
    Attributes(NestedContent),

    /// Explicit newline.
    Newline(PlainContent),

    /// Explicit whitespace.
    Whitespace(PlainContent),

    /// Plain text without any formatting.
    Plain(PlainContent),

    /// Wrapper without any special formatting for multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    Multiple(NestedContent),
}

impl Inline {
    /// creates a new [`Inline`] with the given content and corresponding to the given [`TokenKind`].
    ///
    /// [`Inline`]: self::Inline
    /// [`TokenKind`]: crate::TokenKind
    pub fn new(content: InlineContent<PlainContent, NestedContent>, kind: TokenKind) -> Self {
        match kind {
            TokenKind::Bold => Self::Bold(content.into()),
            TokenKind::Italic => Self::Italic(content.into()),
            TokenKind::Underline => Self::Underline(content.into()),
            TokenKind::Subscript => Self::Subscript(content.into()),
            TokenKind::Superscript => Self::Superscript(content.into()),
            TokenKind::Overline => Self::Overline(content.into()),
            TokenKind::Strikethrough => Self::Strikethrough(content.into()),
            TokenKind::Highlight => Self::Highlight(content.into()),
            TokenKind::Quote => Self::Quote(content.into()),
            TokenKind::Math => Self::Math(content.into()),
            TokenKind::OpenParens => Self::Parens(content.into()),
            TokenKind::OpenBracket => Self::TextGroup(content.into()),
            TokenKind::OpenBrace => Self::Attributes(content.into()),

            TokenKind::Verbatim => Self::Verbatim(content.into()),
            TokenKind::Newline => Self::Newline(content.into()),
            TokenKind::Whitespace => Self::Whitespace(content.into()),
            TokenKind::Plain => match content {
                InlineContent::Plain(plain_content) => Self::Plain(plain_content),
                InlineContent::Nested(nested_content) => Self::Multiple(nested_content),
            },

            // These cases should never be reached
            TokenKind::UnderlineSubscript => Self::Plain(content.into()),
            TokenKind::ItalicBold => Self::Plain(content.into()),
            TokenKind::CloseParens => Self::Plain(content.into()),
            TokenKind::CloseBracket => Self::Plain(content.into()),
            TokenKind::CloseBrace => Self::Plain(content.into()),
        }
    }

    /// creates a new [`Inline`] with the given content updated with the provided [`Span`] and
    /// corresponding to the given [`TokenKind`].
    ///
    /// [`Inline`]: self::Inline
    /// [`TokenKind`]: crate::TokenKind
    /// [`SPan`]: crate::TokenKind
    pub fn with_span(
        mut content: InlineContent<PlainContent, NestedContent>,
        kind: TokenKind,
        span: Span,
    ) -> Self {
        content.set_span(span);
        Self::new(content, kind)
    }

    /// Checks whether this [`Inline`] and `other` are of the same kind.
    ///
    /// [`Inline`]: self::Inline
    pub fn matches_kind(&self, other: &Inline) -> bool {
        match self {
            Inline::Bold(_) => matches!(other, Self::Bold(_)),
            Inline::Italic(_) => matches!(other, Self::Italic(_)),
            Inline::Underline(_) => matches!(other, Self::Underline(_)),
            Inline::Subscript(_) => matches!(other, Self::Subscript(_)),
            Inline::Superscript(_) => matches!(other, Self::Superscript(_)),
            Inline::Overline(_) => matches!(other, Self::Overline(_)),
            Inline::Strikethrough(_) => matches!(other, Self::Strikethrough(_)),
            Inline::Highlight(_) => matches!(other, Self::Highlight(_)),
            Inline::Verbatim(_) => matches!(other, Self::Verbatim(_)),
            Inline::Quote(_) => matches!(other, Self::Quote(_)),
            Inline::Math(_) => matches!(other, Self::Math(_)),
            Inline::Parens(_) => matches!(other, Self::Parens(_)),
            Inline::TextGroup(_) => matches!(other, Self::TextGroup(_)),
            Inline::Attributes(_) => matches!(other, Self::Attributes(_)),
            Inline::Newline(_) => matches!(other, Self::Newline(_)),
            Inline::Whitespace(_) => matches!(other, Self::Whitespace(_)),
            Inline::Plain(_) => matches!(other, Self::Plain(_) | Self::Multiple(_)),
            Inline::Multiple(_) => matches!(other, Self::Multiple(_) | Self::Plain(_)),
        }
    }

    /// Checks whether this [`Inline`] is a `Plain` text constructed from multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    fn is_multiple(&self) -> bool {
        matches!(self, Inline::Multiple(_))
    }

    /// Consumes this [`Inline`] and returns the inner [`InlineContent`] of it.
    ///
    /// [`Inline`]: self::Inline
    /// [`InlineContent`]: self::InlineContent
    pub fn into_inner(self) -> InlineContent<PlainContent, NestedContent> {
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
            | Inline::Multiple(nested_content)
            | Inline::Attributes(nested_content) => InlineContent::Nested(nested_content),
        }
    }

    /// Returns a textual representation of this [`Inline`] as found in original input.
    ///
    /// [`Inline`]: self::Inline
    pub fn as_string(&self) -> String {
        let token_kind = TokenKind::from(self);
        let (begin_delim, end_delim) = token_kind.delimiters();

        let delim_len = begin_delim.len() + end_delim.len();

        let mut res = String::with_capacity(self.content_len() + delim_len);

        res.push_str(begin_delim);
        res.push_str(&self.as_ref().as_string());
        res.push_str(end_delim);

        res
    }

    /// Returns the length of content of this [`Inline`].
    ///
    /// [`Inline`]: self::Inline
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
            | Inline::Multiple(nested_content)
            | Inline::Attributes(nested_content) => nested_content.content_len(),
        }
    }

    /// Returns the [`Span`] that this [`Inline`] occupies.
    ///
    /// [`Inline`]: self::Inline
    /// [`Span`]: crate::Span
    pub fn span(&self) -> Span {
        self.as_ref().span()
    }

    /// Returns the inner content as an immutable reference.
    pub fn as_ref(&self) -> InlineContent<&PlainContent, &NestedContent> {
        match self {
            Inline::Verbatim(content)
            | Inline::Parens(content)
            | Inline::Newline(content)
            | Inline::Whitespace(content)
            | Inline::Plain(content) => InlineContent::Plain(content),

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
            | Inline::Multiple(content)
            | Inline::Attributes(content) => InlineContent::Nested(content),
        }
    }

    /// Merges this [`Inline`] with another into one combined [`Inline`]. Since the other [`Inline`] might
    /// contain multiple inlines inside, some of which aren't compatible with this one, the remaining [`Inline`]s
    /// are returned in a [`VecDeque`].
    ///
    /// [`Inline`]: self::Inline
    /// [`VecDeque`]: std::collections::VecDeque
    pub(crate) fn merge(self, next_inline: Inline) -> (Inline, VecDeque<Inline>) {
        let kind = TokenKind::from(&self);
        let is_multiple = next_inline.is_multiple();

        let mut current_content = self.into_inner();
        let next_content = next_inline.into_inner();

        let rest_of_inlines = match next_content {
            InlineContent::Plain(plain_content) => {
                // merge plains trivially
                current_content.append(plain_content.into());
                VecDeque::default()
            }
            InlineContent::Nested(nested_inlines) => {
                let mut content = nested_inlines.content;

                while let Some(inline) = content.get(0) {
                    let token_kind = TokenKind::from(inline);
                    let should_append = !is_multiple || token_kind == kind;

                    if should_append {
                        current_content.append_inline(content.pop_front().unwrap());
                    } else {
                        break;
                    }
                }

                content
            }
        };

        let result_inline = Self::new(current_content, kind);
        (result_inline, rest_of_inlines)
    }
}

impl From<PlainContent> for Inline {
    fn from(content: PlainContent) -> Self {
        Self::Plain(content)
    }
}
