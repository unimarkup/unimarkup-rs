use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use crate::{Inline, Span, Token, TokenKind};

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
#[derive(Default, Debug, Clone, PartialEq, Eq)]
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

impl From<Inline> for NestedContent {
    fn from(inline: Inline) -> Self {
        let span = inline.span();
        let content = vec![inline].into();

        NestedContent { content, span }
    }
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
    /// Creates a new `PlainContent` with the given text and [`Span`].
    ///
    /// [`Span`]: crate::Span
    pub fn new(content: String, span: Span) -> Self {
        Self { content, span }
    }

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
        let start = other.span().start();
        let end = self.span().end();

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

        InlineContent::Plain(PlainContent::new(content, span))
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
            InlineContent::Nested(nested_content) => {
                PlainContent::new(nested_content.as_string(), nested_content.span)
            }
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
        Self::Plain(PlainContent::new(
            String::from(token.as_str()),
            token.span(),
        ))
    }
}
