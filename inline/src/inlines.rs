use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use crate::{Span, Token, TokenKind};

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
                let mut content = VecDeque::from(nested_inlines.content);

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

/// Content of an [`Inline`].
///
/// [`Inline`]: self::Inline
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineContent<Plain, Nested> {
    /// Simple non-nested content of an inline, like [`Inline::Plain`].
    ///
    /// [`Inline::Plain`]: self::Inline::Plain
    Plain(Plain),

    /// Nested content, might consist of multiple other [`Inline`]s.
    ///
    /// [`Inline`]: self::Inline
    Nested(Nested),
}

/// Nested content of an [`Inline`] consisting of multiple other [`Inline`].
///
/// [`Inline`]: self::Inline
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedContent {
    pub(crate) content: VecDeque<Inline>,
    pub(crate) span: Span,
}

/// Plain content of an [`Inline`] consisting of simple text.
///
/// [`Inline`]: self::Inline
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PlainContent {
    pub(crate) content: String,
    pub(crate) span: Span,
}

impl NestedContent {
    /// Returns the combined length of all [`Inline`]s contained.
    ///
    /// [`Inline`]: self::Inline
    pub fn content_len(&self) -> usize {
        self.content.iter().map(Inline::content_len).sum()
    }

    /// Returns a textual representation of inner [`Inline`]s combined.
    ///
    /// [`Inline`]: self::Inline
    pub fn as_string(&self) -> String {
        self.content.iter().map(Inline::as_string).collect()
    }

    /// Returns the number of [`Inline`]s contained.
    ///
    /// [`Inline`]: self::Inline
    pub fn count(&self) -> usize {
        self.content.len()
    }
}

impl Index<usize> for NestedContent {
    type Output = Inline;

    fn index(&self, index: usize) -> &Self::Output {
        self.content.get(index).unwrap()
    }
}

impl IndexMut<usize> for NestedContent {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.content.get_mut(index).unwrap()
    }
}

impl PlainContent {
    /// Returns the content as [`&str`].
    ///
    /// [`&str`]: &str
    pub fn as_str(&self) -> &str {
        &self.content
    }

    /// Returns the content as [`String`].
    ///
    /// [String]: String
    pub fn as_string(&self) -> String {
        self.content.clone()
    }

    /// Returns the lenght of the content.
    pub fn content_len(&self) -> usize {
        self.content.len()
    }
}

impl From<InlineContent<PlainContent, NestedContent>> for PlainContent {
    fn from(content: InlineContent<PlainContent, NestedContent>) -> Self {
        content.into_plain()
    }
}

impl From<InlineContent<PlainContent, NestedContent>> for NestedContent {
    fn from(content: InlineContent<PlainContent, NestedContent>) -> Self {
        content.into_nested()
    }
}

impl InlineContent<PlainContent, NestedContent> {
    /// If possible combines multiple consecutive [`Inline`]s of same kind.
    ///
    /// [`Inline`]: self::Inline
    pub fn try_flatten(&mut self) {
        if let InlineContent::Nested(nested_inlines) = self {
            if nested_inlines.content.is_empty() {
                return;
            }

            let curr_content = std::mem::take(&mut nested_inlines.content);

            let mut res_vec: VecDeque<Inline> = VecDeque::with_capacity(curr_content.len());

            for inline in curr_content {
                let matches_prev = res_vec
                    .back()
                    .map_or(false, |prev_inline| prev_inline.matches_kind(&inline));

                if matches_prev {
                    if let Some(prev_inline) = res_vec.pop_back() {
                        let token_kind = TokenKind::from(&prev_inline);

                        let mut prev_content = prev_inline.into_inner();
                        prev_content.append_inline(inline);

                        res_vec.push_back(Inline::new(prev_content, token_kind));
                    }
                } else {
                    res_vec.push_back(inline);
                }
            }

            nested_inlines.content = res_vec;
        }
    }

    /// Prepends another [`InlineContent`] to this [`InlineContent`].
    ///
    /// [`InlineContent`]: self::InlineContent
    pub fn prepend(&mut self, other: InlineContent<PlainContent, NestedContent>) {
        let start = other.span().end();
        let end = self.span().start();

        match self {
            InlineContent::Plain(plain_content) => match other {
                InlineContent::Plain(mut other_plain) => {
                    std::mem::swap(&mut plain_content.content, &mut other_plain.content);

                    plain_content.content.push_str(&other_plain.content);
                }
                InlineContent::Nested(mut other_inlines) => {
                    other_inlines
                        .content
                        .push_back(Inline::from(std::mem::take(plain_content)));

                    *self = InlineContent::Nested(other_inlines);
                }
            },

            InlineContent::Nested(self_nested) => match other {
                InlineContent::Plain(other_plain) => {
                    self_nested.content.push_front(Inline::from(other_plain));
                }
                InlineContent::Nested(mut other_inlines) => {
                    std::mem::swap(&mut self_nested.content, &mut other_inlines.content);
                    self_nested.content.append(&mut other_inlines.content);
                }
            },
        }

        self.set_span(Span::from((start, end)));
    }

    /// Apends an [`Inline`] to this content.
    ///
    /// [`Inline`]: self::Inline
    pub fn append_inline(&mut self, inline: Inline) {
        let start = self.span().start();
        let end = inline.span().end();

        match self {
            InlineContent::Plain(ref mut plain_content) => {
                // From inline definitions, this should not be possible. Every variant has already
                // specified inline content type as it's inner value. Therefore, if some inline has
                // plain as content, then it can't have nested content. append the inline as text
                // to the current inline is the solution.
                plain_content.content.push_str(&inline.as_string());
            }
            InlineContent::Nested(ref mut nested_inlines) => {
                nested_inlines.content.push_back(inline)
            }
        }

        self.set_span(Span::from((start, end)));
    }

    /// Appends another [`InlineContent`] to this [`InlineContent`].
    ///
    /// [`InlineContent`]: self::InlineContent
    pub fn append(&mut self, mut other: InlineContent<PlainContent, NestedContent>) {
        let span = (self.span().start(), other.span().end()).into();

        match self {
            InlineContent::Plain(plain_content) => match other {
                InlineContent::Plain(ref other_plain) => {
                    plain_content.content.push_str(&other_plain.content);
                }
                InlineContent::Nested(ref mut other_inlines) => {
                    let mut content = std::mem::take(&mut other_inlines.content);
                    content.push_front(Inline::from(std::mem::take(plain_content)));

                    *self = InlineContent::Nested(NestedContent { content, span });
                }
            },

            InlineContent::Nested(nested_inlines) => match other {
                InlineContent::Plain(plain_content) => {
                    nested_inlines
                        .content
                        .push_back(Inline::from(plain_content));
                }

                InlineContent::Nested(ref mut other_inlines) => {
                    nested_inlines.content.append(&mut other_inlines.content);
                }
            },
        }

        self.set_span(span);
    }

    /// Creates a [`InlineContent::Plain`] from any given [`Token`], discarding it's [`TokenKind`].
    ///
    /// [`InlineContent::Plain`]: self::InlineContent::Plain
    /// [`Token`]: crate::Token
    /// [`TokenKind`]: crate::TokenKind
    pub fn from_token_as_plain(token: Token) -> Self {
        let content = String::from(token.as_str());
        let span = token.span();

        InlineContent::Plain(PlainContent { content, span })
    }

    /// Returns the span that this content occupies.
    pub fn span(&self) -> Span {
        match self {
            InlineContent::Plain(plain_content) => plain_content.span,
            InlineContent::Nested(nested_inlines) => nested_inlines.span,
        }
    }

    /// Returns the textual representation of content.
    pub fn as_string(&self) -> String {
        match self {
            InlineContent::Plain(content) => content.as_str().to_string(),
            InlineContent::Nested(nested_content) => nested_content.as_string(),
        }
    }

    /// Returns the lenght of content.
    pub fn content_len(&self) -> usize {
        match self {
            InlineContent::Plain(plain_content) => plain_content.content_len(),
            InlineContent::Nested(nested_inlines) => nested_inlines.content_len(),
        }
    }

    /// Converts self into [`PlainContent`], with any inline contained inside of self converted into
    /// the original textual representation.
    ///
    /// [`PlainContent`]: self::PlainContent
    pub fn into_plain(self) -> PlainContent {
        match self {
            InlineContent::Plain(plain_content) => plain_content,
            InlineContent::Nested(nested_content) => PlainContent {
                content: nested_content.as_string(),
                span: nested_content.span,
            },
        }
    }

    /// Converts self into [`NestedContent`].
    ///
    /// [`NestedContent`]: self::NestedContent
    pub fn into_nested(self) -> NestedContent {
        match self {
            InlineContent::Plain(plain_content) => {
                let span = plain_content.span;
                let content = VecDeque::from(vec![Inline::from(plain_content)]);
                NestedContent { content, span }
            }

            InlineContent::Nested(nested_inlines) => nested_inlines,
        }
    }

    /// Updates the [`Span`] that this content occupies.
    ///
    /// [`Span`]: crate::Span
    pub(crate) fn set_span(&mut self, span: Span) {
        match self {
            InlineContent::Plain(ref mut plain_content) => plain_content.span = span,
            InlineContent::Nested(ref mut nested_content) => nested_content.span = span,
        }
    }
}

impl InlineContent<&PlainContent, &NestedContent> {
    /// Returns the span that this content occupies.
    pub fn span(&self) -> Span {
        match self {
            InlineContent::Plain(plain_content) => plain_content.span,
            InlineContent::Nested(nested_inlines) => nested_inlines.span,
        }
    }

    /// Returns the textual representation of content.
    pub fn as_string(&self) -> String {
        match self {
            InlineContent::Plain(content) => content.as_str().to_string(),
            InlineContent::Nested(nested_content) => nested_content.as_string(),
        }
    }

    /// Returns the combined length of the content.
    pub fn content_len(&self) -> usize {
        match self {
            InlineContent::Plain(plain_content) => plain_content.content_len(),
            InlineContent::Nested(nested_inlines) => nested_inlines.content_len(),
        }
    }
}

impl<T> From<NestedContent> for InlineContent<T, NestedContent> {
    fn from(content: NestedContent) -> Self {
        Self::Nested(content)
    }
}

impl<T> From<PlainContent> for InlineContent<PlainContent, T> {
    fn from(content: PlainContent) -> Self {
        Self::Plain(content)
    }
}

impl From<Token> for InlineContent<PlainContent, NestedContent> {
    fn from(token: Token) -> Self {
        Self::Plain(PlainContent {
            content: String::from(token.as_str()),
            span: token.span(),
        })
    }
}
